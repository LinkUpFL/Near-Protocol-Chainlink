use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult, ext_contract};
use near_sdk::wee_alloc::{WeeAlloc};
use near_sdk::serde_json::{self, json};
use std::str;
use std::convert::TryInto;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13

pub type Base64String = String;

#[ext_contract(link_token_contract)]
pub trait LinkTokenContract {
    fn new(owner_id: AccountId, total_supply: U128);
    fn transfer(new_owner_id: AccountId, amount: U128);
}

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

    /**
   * @notice set up the aggregator with initial configuration
   * @param _link The address of the LINK token
   * @param _paymentAmount The amount paid of LINK paid to each oracle per submission, in wei (units of 10⁻¹⁸ LINK)
   * @param _timeout is the number of seconds after the previous round that are
   * allowed to lapse before allowing an oracle to skip an unfinished round
   * @param _validator is an optional contract address for validating
   * external validation of answers
   * @param _minSubmissionValue is an immutable check for a lower bound of what
   * submission values are accepted from an oracle
   * @param _maxSubmissionValue is an immutable check for an upper bound of what
   * submission values are accepted from an oracle
   * @param _decimals represents the number of decimals to offset the answer by
   * @param _description a short description of what is being reported
   */
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

        // Subtraction overlflow error at runtime
        /*let updated_at_insert: u64 = (env::block_timestamp() - timeout_u64) as u64;
        let newRound: Round = Round {
            answer: 0_u128,
            started_at: 0_u64,
            updated_at: updated_at_insert,
            answered_in_round: 0_u64
        };
        result.rounds.insert(&0, &newRound);
        */
        result
    }

}
