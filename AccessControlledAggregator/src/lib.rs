use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use near_sdk::wee_alloc::{WeeAlloc};
use near_sdk::serde_json::{self, json};
use std::str;
use std::convert::TryInto;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13

pub type Base64String = String;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Round {
    answer: u128,
    started_at: u64,
    updated_at: u64,
    answered_in_round: u64
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RoundDetails {
    submissions: Vec<u128>,
    max_submissions: u64,
    min_submissions: u64,
    timeout: u64,
    payment_amount: u128
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OracleStatus {
    withdrawable: u128,
    starting_round: u64,
    ending_round: u128,
    last_reported_round: u64,
    last_started_round: u64,
    latest_submission: u128,
    index: u64,
    admin: AccountId,
    pending_admin: AccountId
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Requester {
    authorized: bool,
    delay: u64,
    last_started_round: u64
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Funds {
    available: u128,
    allocated: u128
}

const VERSION: u128 = 3;
const RESERVE_ROUNDS: u128 = 2;
const MAX_ORACLE_COUNT: u128 = 77;
const ROUND_MAX: u128 = 4294967295; // 2**32-1
const V3_NO_DATA_ERROR: &str = "No data present";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccessControlledAggregator {
    pub owner: AccountId,
    pub link_token: AccountId,
    pub validator: AccountId,
    pub payment_amount: u128,
    pub max_submission_count: u64,
    pub min_submission_count: u64,
    pub restart_delay: u64,
    pub timeout: u64,
    pub decimals: u64,
    pub description: Base64String,
    pub min_submission_value: u128,
    pub max_submission_value: u128,
    pub check_enabled: bool,
    access_list: LookupMap<AccountId, bool>,
    reporting_round_id: u64,
    latest_round_id: u64,
    oracles: LookupMap<AccountId, OracleStatus>,
    rounds: LookupMap<u64, Round>,
    details: LookupMap<u128, RoundDetails>,
    requesters: LookupMap<AccountId, Requester>,
    oracle_addresses: Vec<AccountId>,
    recorded_funds: Funds
}

impl Default for AccessControlledAggregator {
    fn default() -> Self {
        panic!("AccessControlledAggregator should be initialized before usage");
    }
}

#[near_bindgen]
impl AccessControlledAggregator {
    #[init]
    pub fn new(link_id: AccountId, owner_id: AccountId, _payment_amount: U128, _timeout: U64, _validator: AccountId, _min_submission_value: U128, _max_submission_value: U128, _decimals: U64, _description: Base64String) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(link_id.as_bytes()), "Link token account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let payment_amount_u128: u128 = _payment_amount.into();
        let timeout_u64: u64 = _timeout.into();
        let min_submission_value_u128: u128 = _min_submission_value.into();
        let max_submission_value_u128: u128 = _max_submission_value.into();
        let decimals_u64: u64 = _decimals.into();
        let vector: Vec::<AccountId> = Vec::new();

        let mut result = Self {
            owner: owner_id,
            link_token: link_id,
            validator: "".to_string(),
            payment_amount: 0_u128,
            max_submission_count: 0_u64,
            min_submission_count: 0_u64,
            restart_delay: 0_u64,
            timeout: 0_u64,
            decimals: decimals_u64,
            description: _description,
            min_submission_value: min_submission_value_u128,
            max_submission_value: max_submission_value_u128,
            check_enabled: true,
            access_list: LookupMap::new(b"access_list".to_vec()),
            reporting_round_id: 0_u64,
            latest_round_id: 0_u64,
            oracles: LookupMap::new(b"oracles".to_vec()),
            rounds: LookupMap::new(b"rounds".to_vec()),
            details: LookupMap::new(b"details".to_vec()),
            requesters: LookupMap::new(b"requesters".to_vec()),
            oracle_addresses: vector,
            recorded_funds: Funds { available: 0_u128, allocated: 0_u128 }
        };
        result.check_enabled = true;

        let round_option = result.rounds.get(&0);
        let mut round = round_option.unwrap();
        round.updated_at = (env::block_timestamp() - timeout_u64) as u64;

        result.update_future_rounds(U128::from(payment_amount_u128), U64::from(0), U64::from(0), U64::from(0), U64::from(timeout_u64));
        result.set_validator(_validator);
        result
    }

    pub fn submit(&mut self, _round_id: U128, _submission: U128) {
        let round_id_u128: u128 = _round_id.into();
        let submission_u128: u128 = _submission.into();
        let error: Base64String = self.validate_oracle_round(env::current_account_id(), round_id_u128 as u64);
        assert!(submission_u128 >= self.min_submission_value, "value below minSubmissionValue");
        assert!(submission_u128 <= self.max_submission_value, "value above maxSubmissionValue");
        assert!(error.len() == 0, error);

        self.oracle_initialize_new_round(round_id_u128 as u64);
        self.record_submission(submission_u128, round_id_u128);
        let (updated, new_answer): (bool, u128) = self.update_round_answer(round_id_u128 as u64);
        self.pay_oracle(round_id_u128 as u64);
        self.delete_round_details(round_id_u128 as u64);
        if updated {
            self.validate_answer(round_id_u128 as u64, new_answer);
        }
    }

    pub fn change_oracles(&mut self, _removed: Vec<AccountId>, _added: Vec<AccountId>, _added_admins: Vec<AccountId>, _min_submissions: U64, _max_submissions: U64, _restart_delay: U64) {
        self.only_owner();

        let min_submissions_u64: u64 = _min_submissions.into();
        let max_submissions_u64: u64 = _max_submissions.into();
        let restart_delay_u64: u64 = _restart_delay.into();

        for i in 0.._removed.len() {
            self.remove_oracle(_removed[i].clone());
        }

        assert!(_added.len() == _added_admins.len(), "need same oracle and admin count");
        assert!((self.oracle_count() as usize + _added.len()) as u128 <= MAX_ORACLE_COUNT, "max oracles allowed");

        for i in 0.._added.len() {
            self.add_oracle(_added[i].clone(), _added_admins[i].clone());
        }

        self.update_future_rounds(U128::from(self.payment_amount), U64::from(min_submissions_u64), U64::from(max_submissions_u64), U64::from(restart_delay_u64), U64::from(self.timeout));
    }

    pub fn update_future_rounds(&mut self, _payment_amount: U128, _min_submissions: U64, _max_submissions: U64, _restart_delay: U64, _timeout: U64) {
        let payment_amount_u128: u128 = _payment_amount.into();
        let min_submissions_u64: u64 = _min_submissions.into();
        let max_submissions_u64: u64 = _max_submissions.into();
        let restart_delay_u64: u64 = _restart_delay.into();
        let timeout_u64: u64 = _timeout.into();

        let oracle_num: u128 = self.oracle_count(); // Save on storage reads
        assert!(max_submissions_u64 >= min_submissions_u64, "max must equal/exceed min");
        assert!(oracle_num >= max_submissions_u64.into(), "max cannot exceed total");
        assert!(oracle_num == 0 || oracle_num > restart_delay_u64.into(), "delay cannot exceed total");
        assert!(self.recorded_funds.available >= self.required_reserve(payment_amount_u128), "insufficient funds for payment");
        if self.oracle_count() > 0 {
            assert!(min_submissions_u64 > 0, "min must be greater than 0")
        }

        self.payment_amount = payment_amount_u128;
        self.min_submission_count = min_submissions_u64;
        self.max_submission_count = max_submissions_u64;
        self.restart_delay = restart_delay_u64;
        self.timeout = timeout_u64;
    }

    pub fn allocated_funds(&self) -> u128 {
        self.recorded_funds.allocated
    }

    pub fn available_funds(&self) -> u128 {
        self.recorded_funds.available
    }

    pub fn update_available_funds(&mut self) {
        let funds: &Funds = &self.recorded_funds;

        let get_balance_promise = env::promise_create(
            env::current_account_id(),
            b"get_balance",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let get_balance_promise_result: Vec<u8> =
            match env::promise_result(get_balance_promise) {
                PromiseResult::Successful(x) => x,
                _ => panic!("Promise with index 0 failed"),
            };
        let link_balance: u64 = serde_json::from_slice(&get_balance_promise_result).unwrap();
        let now_available: u128 = (link_balance - funds.allocated as u64).into();

        if funds.available != now_available {
            self.recorded_funds.available = now_available as u128;
        }
    }

    pub fn oracle_count(&self) -> u128 {
        self.oracle_addresses.len() as u128
    }

    pub fn get_oracles(&self) -> Vec<AccountId> {
        self.oracle_addresses.clone()
    }

    pub fn latest_answer(&self) -> u128 {
        let round_option = self.rounds.get(&self.latest_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();
        round.answer
    }

    pub fn latest_timestamp(&self) -> u64 {
        let round_option = self.rounds.get(&self.latest_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();
        round.updated_at
    }

    pub fn latest_round(&self) -> u64 {
        self.latest_round_id
    }

    pub fn get_answer(&self, _round_id: U128) -> u128 {
        let round_id_u128: u128 = _round_id.into();

        let round_option = self.rounds.get(&(round_id_u128 as u64));
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();

        if self.valid_round_id(round_id_u128) {
            return round.answer;
        }
        return 0;
    }

    pub fn get_timestamp(&self, _round_id: U128) -> u128 {
        let round_id_u128: u128 = _round_id.into();

        let round_option = self.rounds.get(&(round_id_u128 as u64));
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();

        if self.valid_round_id(round_id_u128) {
            return round.answer;
        }
        return 0;
    }

    pub fn get_round_data(&self, _round_id: U64) -> (u64, u128,  u64, u64, u64) {
        let round_id_u64: u64 = _round_id.into();

        let round_option = self.rounds.get(&round_id_u64);
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();

        let r: Round = round;
        assert!(r.answered_in_round > 0 && self.valid_round_id(round_id_u64.into()), V3_NO_DATA_ERROR);

        return(
            round_id_u64,
            r.answer,
            r.started_at,
            r.updated_at,
            r.answered_in_round
        )
    }

    pub fn latest_round_data(&self) -> (u64, u128,  u64, u64, u64) {
        self.get_round_data(U64::from(self.latest_round_id))
    }

    pub fn withdrawable_payment(&self, _oracle: AccountId) -> u128 {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        oracle.withdrawable
    }

    pub fn withdraw_payment(&mut self, _oracle: AccountId, _recipient: AccountId, _amount: U128) {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();
        assert!(oracle.admin == env::predecessor_account_id(), "only callable by admin");

        // Safe to downcast _amount because the total amount of LINK is less than 2^128.
        let amount_u128: u128 = _amount.into();
        let available: u128 = oracle.withdrawable;
        assert!(available >= amount_u128, "insufficient withdrawable funds");

        oracle.withdrawable = available - amount_u128;
        self.oracles.insert(&_oracle, &oracle);
        self.recorded_funds.allocated = self.recorded_funds.allocated - amount_u128;

        //assert(link_token.transfer(_recipient, uint256(amount)));
    }

    pub fn withdraw_funds(&mut self, _recipient: AccountId, _amount: U128) {
        let available: u128 = self.recorded_funds.available as u128;
        let amount_u128: u128 = _amount.into();
        assert!((available - self.required_reserve(self.payment_amount)) >= amount_u128, "insufficient reserve funds");
        // assert link_token transfer
        self.update_available_funds();
    }

    pub fn get_admin(&self, _oracle: AccountId) -> AccountId {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        oracle.admin
    }

    pub fn transfer_admin(&mut self, _oracle: AccountId, _new_admin: AccountId) {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();
        assert!(oracle.admin == env::predecessor_account_id(), "only callable by admin");
        oracle.pending_admin = _new_admin;
        self.oracles.insert(&_oracle, &oracle);
    }

    pub fn accept_admin(&mut self, _oracle: AccountId) {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();
        assert!(oracle.pending_admin == env::predecessor_account_id(), "only callable by pending admin");
        oracle.pending_admin = "".to_string();
        oracle.admin = env::predecessor_account_id();
        self.oracles.insert(&_oracle, &oracle);
    }

    pub fn request_new_round(&mut self) -> u64 {
        let requester_option = self.requesters.get(&env::predecessor_account_id());
        if requester_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let requester = requester_option.unwrap();
        assert!(requester.authorized, "not authorized requester");

        let current: u64 = self.reporting_round_id;
        let round_option = self.rounds.get(&current);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        assert!(round.updated_at > 0 || self.timed_out(current), "prev round must be supersedable");

        let new_round_id: u64 = current + 1;
        self.requester_initialize_new_round(new_round_id);
        return new_round_id;
    }

    pub fn set_requester_permissions(&mut self, _requester: AccountId, _authorized: bool, _delay: U64) {
        let delay_u64: u64 = _delay.into();

        let requester_option = self.requesters.get(&_requester);
        if requester_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut requester = requester_option.unwrap();

        if requester.authorized == _authorized {
            return;
        }

        if _authorized {
            requester.authorized = _authorized;
            requester.delay = delay_u64;
            self.requesters.insert(&_requester, &requester);
        } else {
            self.requesters.remove(&_requester);
        }
    }

    pub fn on_token_transfer(&mut self, _address: AccountId, _num: U128, _data: Base64String) {
        assert!(_data.len() == 0, "transfer doesn't accept calldata");
        self.update_available_funds();
    }

    pub fn oracle_round_state(&mut self, _oracle: AccountId, _queried_round_id: U64) -> (bool, u64, u128, u64, u64, u128, u64, u128) {
        assert!(env::predecessor_account_id() == env::signer_account_id(), "off-chain reading only");

        let queried_round_id_u64: u64 = _queried_round_id.into();

        let round_option = self.rounds.get(&queried_round_id_u64);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let detail_option = self.details.get(&(queried_round_id_u64 as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let detail = detail_option.unwrap();

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let oracle = oracle_option.unwrap();

        if queried_round_id_u64 > 0 {
            let round: Round = round;
            return (
                self.eligible_for_specific_round(_oracle, queried_round_id_u64),
                queried_round_id_u64,
                oracle.latest_submission,
                round.started_at,
                detail.timeout,
                self.recorded_funds.available,
                self.oracle_count() as u64,
                if round.started_at > 0 { detail.payment_amount } else { self.payment_amount }
            )
        } else {
            return self.oracle_round_state_suggest_round(_oracle);
        }
    }

    pub fn set_validator(&mut self, _new_validator: AccountId) {
        let previous: AccountId = String::from(&self.validator) as AccountId;

        if previous != _new_validator {
            self.validator = _new_validator;
        }
    }

    fn initialize_new_round(&mut self, _round_id: u64) {
        self.update_timed_out_round_info(_round_id - 1);
        self.reporting_round_id = _round_id;

        let first_round_option = self.rounds.get(&0);
        if first_round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let first_round = first_round_option.unwrap();

        let round_option = self.rounds.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }

        let mut round: Round = first_round;
        let vector: Vec<u128> = Vec::new();
        let next_details: RoundDetails = RoundDetails {
            submissions: vector,
            max_submissions: self.max_submission_count,
            min_submissions: self.min_submission_count,
            timeout: self.timeout,
            payment_amount: self.payment_amount
        };
        self.details.insert(&(_round_id as u128), &next_details);
        round.started_at = env::block_timestamp() as u64;
    }

    fn oracle_initialize_new_round(&mut self, _round_id: u64) {
        if !self.new_round(_round_id) {
            return;
        }
        let oracle_option = self.oracles.get(&env::predecessor_account_id());
        if oracle_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut oracle = oracle_option.unwrap();

        let last_started: u64 = oracle.last_started_round; // cache storage reads
        if _round_id <= last_started + self.restart_delay && last_started != 0 {
            return;
        }
        self.initialize_new_round(_round_id);
        oracle.last_started_round = _round_id;
        self.oracles.insert(&env::predecessor_account_id(), &oracle);
    }

    fn requester_initialize_new_round(&mut self, _round_id: u64) {
        if !self.new_round(_round_id) {
            return;
        }
        let requester_option = self.requesters.get(&env::predecessor_account_id());
        if requester_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut requester = requester_option.unwrap();

        let last_started: u128 = requester.last_started_round.into(); // cache storage reads
        assert!((_round_id as u128) > last_started + (requester.delay as u128) || last_started == 0, "must delay requests");

        self.initialize_new_round(_round_id);

        requester.last_started_round = _round_id;
        self.requesters.insert(&env::predecessor_account_id(), &requester);
    }

    fn update_timed_out_round_info(&mut self, _round_id: u64) {
        if !self.timed_out(_round_id) {
            return;
        }
        let prev_id: u64 = _round_id - 1;

        let round_option = self.rounds.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut round = round_option.unwrap();

        let prev_option = self.rounds.get(&prev_id);
        if prev_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let prev = prev_option.unwrap();

        round.answer = prev.answer;
        round.answered_in_round = prev.answered_in_round;
        round.updated_at = env::block_timestamp() as u64;
        self.rounds.insert(&_round_id, &round);

        self.details.remove(&(_round_id as u128));
    }

    fn eligible_for_specific_round(&self, _oracle: AccountId, _queried_round_id: u64) -> bool {
        let init_oracle = &_oracle;
        let round_option = self.rounds.get(&_queried_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        if round.started_at > 0 {
            return self.accepting_submissions(_queried_round_id.into()) && self.validate_oracle_round(init_oracle.to_string(), _queried_round_id).len() == 0
        } else {
            return self.delayed(_oracle.to_string(), _queried_round_id) && self.validate_oracle_round(init_oracle.to_string(), _queried_round_id).len() == 0
        }
    }

    fn oracle_round_state_suggest_round(&mut self, _oracle: AccountId) -> (bool, u64, u128, u64, u64, u128, u64, u128) {
        let round_option = self.rounds.get(&0);
        let init_oracle = &_oracle;
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();

        let should_supersede: bool = oracle.last_reported_round == self.reporting_round_id || !self.accepting_submissions(self.reporting_round_id as u128);
        // Instead of nudging oracles to submit to the next round, the inclusion of
        // the should_supersede bool in the if condition pushes them towards
        // submitting in a currently open round.

        let mut _round_id: u64;
        let mut _payment_amount: u128;
        let mut _eligible_to_submit: bool;
        let _reporting_round_id: u64 = self.reporting_round_id;

        let detail_option = self.details.get(&(_reporting_round_id as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let detail = detail_option.unwrap();

        if self.supersedable(self.reporting_round_id) && should_supersede {
            _round_id = self.reporting_round_id + 1;

            let round_from_id_option = self.rounds.get(&_round_id);
            if round_from_id_option.is_none() {
                env::panic(b"Did not find this round.");
            }
            let round_from_id = round_from_id_option.unwrap();
            self.rounds.insert(&0, &round_from_id);

            _payment_amount = self.payment_amount;
            _eligible_to_submit = self.delayed(_oracle.to_string(), _round_id);
        } else {
            _round_id = self.reporting_round_id;
            let round_from_id_option = self.rounds.get(&_reporting_round_id);
            if round_from_id_option.is_none() {
                env::panic(b"Did not find this round.");
            }
            let round_from_id = round_from_id_option.unwrap();
            self.rounds.insert(&0, &round_from_id);

            _payment_amount = detail.payment_amount;
            _eligible_to_submit = self.accepting_submissions(_round_id.into());
        }

        if self.validate_oracle_round(init_oracle.to_string(), _round_id).len() != 0 {
            _eligible_to_submit = false;
        }

        return (
            _eligible_to_submit,
            _round_id,
            oracle.latest_submission,
            round.started_at,
            detail.timeout,
            self.recorded_funds.available,
            self.oracle_count() as u64,
            _payment_amount
        );
    }

    fn update_round_answer(&mut self, _round_id: u64) -> (bool, u128) {
        let detail_option = self.details.get(&(_round_id as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        let round_option = self.rounds.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut round = round_option.unwrap();

        let new_answer: u128 = self.median(detail.submissions).into();
        round.answer = new_answer;
        round.updated_at = env::block_timestamp() as u64;
        round.answered_in_round = _round_id;
        self.rounds.insert(&_round_id, &round);
        self.latest_round_id = _round_id;

        return (true, new_answer);
    }

    fn validate_answer(&self, _round_id: u64, _new_answer: u128) {
        let av: AccountId = self.validator.clone(); // cache storage reads
        if av == "" {
            return;
        }

        let prev_round: u64 = _round_id - 1;

        let round_option = self.rounds.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let prev_answer_round_id: u64 = round.answered_in_round;
        let prev_round_answer: u128 = round.answer;
        // TRY CATCH
    }

    fn pay_oracle(&mut self, _round_id: u64) {
        let detail_option = self.details.get(&(_round_id as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        let oracle_option = self.oracles.get(&env::predecessor_account_id());
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();

        let payment: u128 = detail.payment_amount;
        let mut funds: Funds = self.recorded_funds.clone();
        funds.available = funds.available - payment;
        funds.allocated = funds.allocated - payment;
        self.recorded_funds = funds;
        oracle.withdrawable = oracle.withdrawable + payment;
        self.oracles.insert(&env::predecessor_account_id(), &oracle);
    }

    fn record_submission(&mut self, _submission: u128, _round_id: u128) {
        assert!(self.accepting_submissions(_round_id), "round not accepting submissions");

        let detail_option = self.details.get(&(_round_id as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut detail = detail_option.unwrap();

        let oracle_option = self.oracles.get(&env::predecessor_account_id());
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();

        detail.submissions.push(_submission);
        self.details.insert(&(_round_id as u128), &detail);

        oracle.last_reported_round = _round_id as u64;
        oracle.latest_submission = _submission;
        self.oracles.insert(&env::predecessor_account_id(), &oracle);
    }

    fn delete_round_details(&mut self, _round_id: u64) {
        let detail_option = self.details.get(&(_round_id as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        if (detail.submissions.len() as u64) < detail.max_submissions {
            return;
        }

        self.details.remove(&(_round_id as u128));
    }

    fn timed_out(&mut self, _round_id: u64) -> bool {
        let round_option = self.rounds.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let detail_option = self.details.get(&(_round_id as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        let started_at: u64 = round.started_at;
        let round_timeout: u64 = detail.timeout;
        return started_at > 0 && round_timeout > 0 && ((started_at + round_timeout) < env::block_timestamp());
    }

    fn get_starting_round(&self, _oracle: AccountId) -> u64 {
        let current_round: u64 = self.reporting_round_id;
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();

        if current_round != 0 && current_round == oracle.ending_round as u64 {
            return current_round;
        }
        return current_round + 1;
    }

    fn previous_and_current_unanswered(&self, _round_id: u64, _rr_id: u64) -> bool {
        let round_option = self.rounds.get(&_rr_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        return (_round_id + 1) == _rr_id && round.updated_at == 0;
    }

    fn required_reserve(&self, payment: u128) -> u128 {
        return payment * (self.oracle_count() * RESERVE_ROUNDS);
    }

    fn add_oracle(&mut self, _oracle: AccountId, _admin: AccountId) {
        let init_oracle = &_oracle;
        assert!(!self.oracle_enabled(init_oracle.to_string()), "oracle already enabled");
        assert!(_admin != "", "cannot set admin to 0");

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();

        assert!(oracle.admin == env::predecessor_account_id() || oracle.admin == _admin, "owner cannot overwrite admin");

        oracle.starting_round = self.get_starting_round(init_oracle.to_string());
        oracle.ending_round = ROUND_MAX;
        oracle.index = self.oracle_addresses.len() as u64;
        self.oracle_addresses.push(init_oracle.to_string());
        oracle.admin = _admin;
        self.oracles.insert(&_oracle, &oracle);
    }

    fn remove_oracle(&mut self, _oracle: AccountId) {
        let init_oracle = &_oracle;
        assert!(self.oracle_enabled(init_oracle.to_string()), "oracle not enabled");

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();

        let last_oracle: usize = (self.oracle_count() - 1) as usize;
        let tail: AccountId = self.oracle_addresses[last_oracle].clone();
        let init_tail = &tail;

        let oracle_tail_option = self.oracles.get(&tail);
        if oracle_tail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle_tail = oracle_tail_option.unwrap();

        oracle.ending_round = (self.reporting_round_id + 1).into();
        let index: usize = oracle.index.try_into().unwrap();
        oracle_tail.index = index.try_into().unwrap();
        oracle.index = 0_u64;
        self.oracle_addresses[index] = init_tail.to_string();
        self.oracle_addresses.pop();

        self.oracles.insert(&_oracle, &oracle);
        self.oracles.insert(&tail, &oracle_tail);
    }

    fn validate_oracle_round(&self, _oracle: AccountId, _round_id: u64) -> Base64String {
        // cache storage reads
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        let starting_round: u64 = oracle.starting_round;
        let rr_id: u64 = self.reporting_round_id;

        if starting_round == 0 {
            return "not enabled oracle".to_string();
        }
        else if starting_round > _round_id {
            return "not yet enabled oracle".to_string();
        }
        else if oracle.ending_round < _round_id.into() {
            return "no longer allowed oracle".to_string();
        }
        else if oracle.last_reported_round >= _round_id {
            return "cannot report on previous rounds".to_string();
        }
        else if _round_id != rr_id && _round_id != rr_id + 1 && !self.previous_and_current_unanswered(_round_id, rr_id) {
            return "invalid round to report".to_string();
        }
        else {
            return "previous round not supersedable".to_string();
        }
    }
    fn supersedable(&mut self, _round_id: u64) -> bool {
        let round_option = self.rounds.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        round.updated_at > 0 || self.timed_out(_round_id)
    }

    fn oracle_enabled(&self, _oracle: AccountId) -> bool {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        oracle.ending_round == ROUND_MAX
    }

    fn accepting_submissions(&self, _round_id: u128) -> bool {
        let round_option = self.details.get(&_round_id);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        round.max_submissions != 0
    }

    fn delayed(&self, _oracle: AccountId, _round_id: u64) -> bool {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        let last_started: u64 = oracle.last_started_round;
        _round_id > (last_started + self.restart_delay) || last_started == 0
    }

    fn new_round(&self, _round_id: u64) -> bool {
        _round_id == self.reporting_round_id + 1
    }

    fn valid_round_id(&self, _round_id: u128) -> bool {
        _round_id <= ROUND_MAX
    }

    fn only_owner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }

    fn median(&mut self, mut numbers: Vec<u128>) -> u128 {
        numbers.sort();
        let mid = numbers.len() / 2;
        numbers[mid]
    }

    pub fn get_decimals(&self) -> u64 {
        self.decimals
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_version(&self) -> u128 {
        VERSION
    }

    // Access Control

    pub fn has_access(&self, _user: AccountId) -> bool {
        if !self.check_enabled {
            !self.check_enabled
        } else {
            let user_option = self.access_list.get(&_user);
            if user_option.is_none() {
                env::panic(b"Did not find this oracle account.");
            }
            let user = user_option.unwrap();
            user
        }
    }

    pub fn add_access(&mut self, _user: AccountId) {
        self.only_owner();

        let user_option = self.access_list.get(&_user);
        if user_option.is_none() {
            self.access_list.insert(&_user, &true);
            env::panic(b"Added access to this oracle account.");
        }
    }

    pub fn remove_access(&mut self, _user: AccountId) {
        self.only_owner();

        let user_option = self.access_list.get(&_user);
        if user_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        self.access_list.insert(&_user, &false);
    }

    pub fn enable_access_check(&mut self) {
        self.only_owner();

        if !self.check_enabled {
            self.check_enabled = true;
        }
    }

    pub fn disable_access_check(&mut self) {
        self.only_owner();

        if self.check_enabled {
            self.check_enabled = false;
        }
    }

    fn check_access(&self) {
        assert!(self.has_access(env::predecessor_account_id()), "No access")
    }
}
