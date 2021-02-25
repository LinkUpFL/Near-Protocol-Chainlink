use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use std::str;
use std::convert::TryInto;
use num_traits::pow;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub type Base64String = String;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Round {
    answer: u128,
    startedAt: u64,
    updatedAt: u64,
    answeredInRound: u64
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RoundDetails {
    submissions: Vec<u128>,
    maxSubmissions: u64,
    minSubmissions: u64,
    timeout: u64,
    paymentAmount: u128
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OracleStatus {
    withdrawable: u128,
    startingRound: u64,
    endingRound: u128,
    lastReportedRound: u64,
    lastStartedRound: u64,
    latestSubmission: u128,
    index: u64,
    admin: AccountId,
    pendingAdmin: AccountId
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Requester {
    authorized: bool,
    delay: u64,
    lastStartedRound: u64
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Funds {
    available: u128,
    allocated: u128
}

const version: u128 = 3;
const RESERVE_ROUNDS: u128 = 2;
const MAX_ORACLE_COUNT: u128 = 77;
// Previous: 2.pow(32-1)
const ROUND_MAX: u128 = pow(32-1, 2);
const V3_NO_DATA_ERROR: Base64String = "No data present".to_string();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccessControlledAggregator {
    pub owner: AccountId,
    pub linkToken: AccountId,
    pub validator: AccountId,
    pub paymentAmount: u128,
    pub maxSubmissionCount: u64,
    pub minSubmissionCount: u64,
    pub restartDelay: u64,
    pub timeout: u64,
    pub decimals: u64,
    pub description: Base64String,
    pub minSubmissionValue: u128,
    pub maxSubmissionValue: u128,
    pub checkEnabled: bool,
    accessList: LookupMap<AccountId, bool>,
    reportingRoundId: u64,
    latestRoundId: u64,
    oracles: LookupMap<AccountId, OracleStatus>,
    rounds: LookupMap<u64, Round>,
    details: LookupMap<u128, RoundDetails>,
    requesters: LookupMap<AccountId, Requester>,
    oracleAddresses: Vec<AccountId>,
    recordedFunds: Funds
}

impl Default for AccessControlledAggregator {
    fn default() -> Self {
        panic!("AccessControlledAggregator should be initialized before usage");
    }
}

#[near_bindgen]
impl AccessControlledAggregator {
    #[init]
    pub fn new(link_id: AccountId, owner_id: AccountId, _paymentAmount: U128, _timeout: U64, _validator: AccountId, _minSubmissionValue: U128, _maxSubmissionValue: U128, _decimals: U64, _description: Base64String) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(link_id.as_bytes()), "Link token account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let paymentAmount_u128: u128 = _paymentAmount.into();
        let timeout_u64: u64 = _timeout.into();
        let minSubmissionValue_u128: u128 = _minSubmissionValue.into();
        let maxSubmissionValue_u128: u128 = _maxSubmissionValue.into();
        let decimals_u64: u64 = _decimals.into();
        let vector: Vec::<AccountId> = Vec::new();

        let mut result = Self {
            owner: owner_id,
            linkToken: link_id,
            validator: "".to_string(),
            paymentAmount: 0_u128,
            maxSubmissionCount: 0_u64,
            minSubmissionCount: 0_u64,
            restartDelay: 0_u64,
            timeout: 0_u64,
            decimals: decimals_u64,
            description: _description,
            minSubmissionValue: minSubmissionValue_u128,
            maxSubmissionValue: maxSubmissionValue_u128,
            checkEnabled: true,
            accessList: LookupMap::new(b"access_list".to_vec()),
            reportingRoundId: 0_u64,
            latestRoundId: 0_u64,
            oracles: LookupMap::new(b"oracles".to_vec()),
            rounds: LookupMap::new(b"rounds".to_vec()),
            details: LookupMap::new(b"details".to_vec()),
            requesters: LookupMap::new(b"requesters".to_vec()),
            oracleAddresses: vector,
            recordedFunds: Funds { available: 0_u128, allocated: 0_u128 }
        };
        result.checkEnabled = true;

        let round_option = result.rounds.get(&0);
        let mut round = round_option.unwrap();
        round.updatedAt = (env::block_timestamp() - timeout_u64) as u64;

        result.updateFutureRounds(U128::from(paymentAmount_u128), U64::from(0), U64::from(0), U64::from(0), U64::from(timeout_u64));
        result.setValidator(_validator);
        result
    }

    pub fn submit(&mut self, _roundId: U128, _submission: U128) {
        let roundId_u128: u128 = _roundId.into();
        let submission_u128: u128 = _submission.into();
        let error: Base64String = self.validateOracleRound(env::current_account_id(), roundId_u128 as u64);
        assert!(submission_u128 >= self.minSubmissionValue, "value below minSubmissionValue");
        assert!(submission_u128 <= self.maxSubmissionValue, "value above maxSubmissionValue");
        assert!(error.len() == 0, error);

        self.oracleInitializeNewRound(roundId_u128 as u64);
        self.recordSubmission(submission_u128, roundId_u128);
        let (updated, newAnswer): (bool, u128) = self.updateRoundAnswer(roundId_u128 as u64);
        self.payOracle(roundId_u128 as u64);
        self.deleteRoundDetails(roundId_u128 as u64);
        if updated {
            self.validateAnswer(roundId_u128 as u64, newAnswer);
        }
    }

    pub fn changeOracles(&mut self, _removed: Vec<AccountId>, _added: Vec<AccountId>, _addedAdmins: Vec<AccountId>, _minSubmissions: U64, _maxSubmissions: U64, _restartDelay: U64) {
        self.onlyOwner();

        let minSubmissions_u64: u64 = _minSubmissions.into();
        let maxSubmissions_u64: u64 = _maxSubmissions.into();
        let restartDelay_u64: u64 = _restartDelay.into();

        for i in 0.._removed.len() {
            self.removeOracle(_removed[i]);
        }

        assert!(_added.len() == _addedAdmins.len(), "need same oracle and admin count");
        assert!((self.oracleCount() as usize + _added.len()) as u128 <= MAX_ORACLE_COUNT, "max oracles allowed");

        for i in 0.._added.len() {
            self.addOracle(_added[i], _addedAdmins[i]);
        }

        self.updateFutureRounds(U128::from(self.paymentAmount), U64::from(minSubmissions_u64), U64::from(maxSubmissions_u64), U64::from(restartDelay_u64), U64::from(self.timeout));
    }

    pub fn updateFutureRounds(&mut self, _paymentAmount: U128, _minSubmissions: U64, _maxSubmissions: U64, _restartDelay: U64, _timeout: U64) {
        let paymentAmount_u128: u128 = _paymentAmount.into();
        let minSubmissions_u64: u64 = _minSubmissions.into();
        let maxSubmissions_u64: u64 = _maxSubmissions.into();
        let restartDelay_u64: u64 = _restartDelay.into();
        let timeout_u64: u64 = _timeout.into();

        let oracleNum: u128 = self.oracleCount(); // Save on storage reads
        assert!(maxSubmissions_u64 >= minSubmissions_u64, "max must equal/exceed min");
        assert!(oracleNum >= maxSubmissions_u64.into(), "max cannot exceed total");
        assert!(oracleNum == 0 || oracleNum > restartDelay_u64.into(), "delay cannot exceed total");
        assert!(self.recordedFunds.available >= self.requiredReserve(paymentAmount_u128), "insufficient funds for payment");
        if self.oracleCount() > 0 {
            assert!(minSubmissions_u64 > 0, "min must be greater than 0")
        }

        self.paymentAmount = paymentAmount_u128;
        self.minSubmissionCount = minSubmissions_u64;
        self.maxSubmissionCount = maxSubmissions_u64;
        self.restartDelay = restartDelay_u64;
        self.timeout = timeout_u64;
    }

    pub fn allocatedFunds(&self) -> u128 {
        self.recordedFunds.allocated
    }

    pub fn availableFunds(&self) -> u128 {
        self.recordedFunds.available
    }

    pub fn updateAvailableFunds(&mut self) {
        let funds: Funds = self.recordedFunds;

        // uint256 nowAvailable = linkToken.balanceOf(address(this)).sub(funds.allocated);
        let nowAvailable: u128 = funds.available - funds.allocated;
        
        if funds.available != nowAvailable {
            self.recordedFunds.available = nowAvailable as u128;
        }
    }

    pub fn oracleCount(&self) -> u128 {
        self.oracleAddresses.len() as u128
    }

    pub fn getOracles(&self) -> Vec<AccountId> {
        self.oracleAddresses
    }

    pub fn latestAnswer(&self) -> u128 {
        let round_option = self.rounds.get(&self.latestRoundId);
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();
        round.answer
    }

    pub fn latestTimestamp(&self) -> u64 {
        let round_option = self.rounds.get(&self.latestRoundId);
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();
        round.updatedAt
    }

    pub fn latestRound(&self) -> u64 {
        self.latestRoundId
    }

    pub fn getAnswer(&self, _roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();

        let round_option = self.rounds.get(&(roundId_u128 as u64));
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();

        if self.validRoundId(roundId_u128) {
            return round.answer;
        }
        return 0;
    }

    pub fn getTimestamp(&self, _roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();

        let round_option = self.rounds.get(&(roundId_u128 as u64));
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();

        if self.validRoundId(roundId_u128) {
            return round.answer;
        }
        return 0;
    }

    pub fn getRoundData(&self, _roundId: U64) -> (u64, u128,  u64, u64, u64) {
        let roundId_u64: u64 = _roundId.into();

        let round_option = self.rounds.get(&roundId_u64);
        if round_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let round = round_option.unwrap();

        let r: Round = round;
        assert!(r.answeredInRound > 0 && self.validRoundId(roundId_u64.into()), V3_NO_DATA_ERROR);

        return(
            roundId_u64,
            r.answer,
            r.startedAt,
            r.updatedAt,
            r.answeredInRound
        )
    }

    pub fn latestRoundData(&self) -> (u64, u128,  u64, u64, u64) {
        self.getRoundData(U64::from(self.latestRoundId))
    }

    pub fn withdrawablePayment(&self, _oracle: AccountId) -> u128 {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        oracle.withdrawable
    }

    pub fn withdrawPayment(&mut self, _oracle: AccountId, _recipient: AccountId, _amount: U128) {
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
        self.recordedFunds.allocated = self.recordedFunds.allocated - amount_u128;

        //assert(linkToken.transfer(_recipient, uint256(amount)));
    }

    pub fn withdrawFunds(&mut self, _recipient: AccountId, _amount: U128) {
        let available: u128 = self.recordedFunds.available as u128;
        let amount_u128: u128 = _amount.into();
        assert!((available - self.requiredReserve(self.paymentAmount)) >= amount_u128, "insufficient reserve funds");
        // assert linktoken transfer
        self.updateAvailableFunds();
    }

    pub fn getAdmin(&self, _oracle: AccountId) -> AccountId {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        oracle.admin
    }

    pub fn transferAdmin(&mut self, _oracle: AccountId, _newAdmin: AccountId) {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();
        assert!(oracle.admin == env::predecessor_account_id(), "only callable by admin");
        oracle.pendingAdmin = _newAdmin;
        self.oracles.insert(&_oracle, &oracle);
    }

    pub fn acceptAdmin(&mut self, _oracle: AccountId) {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();
        assert!(oracle.pendingAdmin == env::predecessor_account_id(), "only callable by pending admin");
        oracle.pendingAdmin = "".to_string();
        oracle.admin = env::predecessor_account_id();
        self.oracles.insert(&_oracle, &oracle);
    }

    pub fn requestNewRound(&mut self) -> u64 {
        let requester_option = self.requesters.get(&env::predecessor_account_id());
        if requester_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let requester = requester_option.unwrap();
        assert!(requester.authorized, "not authorized requester");

        let current: u64 = self.reportingRoundId;
        let round_option = self.rounds.get(&current);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        assert!(round.updatedAt > 0 || self.timedOut(current), "prev round must be supersedable");

        let newRoundId: u64 = current + 1;
        self.requesterInitializeNewRound(newRoundId);
        return newRoundId;
    }

    pub fn setRequesterPermissions(&mut self, _requester: AccountId, _authorized: bool, _delay: U64) {
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

    pub fn onTokenTransfer(&mut self, _address: AccountId, _num: U128, _data: Base64String) {
        assert!(_data.len() == 0, "transfer doesn't accept calldata");
        self.updateAvailableFunds();
    }

    pub fn oracleRoundState(&mut self, _oracle: AccountId, _queriedRoundId: U64) -> (bool, u64, u128, u64, u64, u128, u64, u128) {
        assert!(env::predecessor_account_id() == env::signer_account_id(), "off-chain reading only");

        let queriedRoundId_u64: u64 = _queriedRoundId.into();

        let round_option = self.rounds.get(&queriedRoundId_u64);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let detail_option = self.details.get(&(queriedRoundId_u64 as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let detail = detail_option.unwrap();

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let oracle = oracle_option.unwrap();

        if queriedRoundId_u64 > 0 {
            let round: Round = round;
            return (
                self.eligibleForSpecificRound(_oracle, queriedRoundId_u64),
                queriedRoundId_u64,
                oracle.latestSubmission,
                round.startedAt,
                detail.timeout,
                self.recordedFunds.available,
                self.oracleCount() as u64,
                if round.startedAt > 0 { detail.paymentAmount } else { self.paymentAmount }
            )
        } else {
            return self.oracleRoundStateSuggestRound(_oracle);
        }
    }

    pub fn setValidator(&mut self, _newValidator: AccountId) {
        let previous: AccountId = self.validator as AccountId;

        if previous != _newValidator {
            self.validator = _newValidator;
        }
    }

    fn initializeNewRound(&mut self, _roundId: u64) {
        self.updateTimedOutRoundInfo(_roundId - 1);
        self.reportingRoundId = _roundId;

        let firstRound_option = self.rounds.get(&0);
        if firstRound_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let firstRound = firstRound_option.unwrap();

        let round_option = self.rounds.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }

        let mut round: Round = firstRound;
        let vector: Vec<u128> = Vec::new();
        let nextDetails: RoundDetails = RoundDetails {
            submissions: vector,
            maxSubmissions: self.maxSubmissionCount,
            minSubmissions: self.minSubmissionCount,
            timeout: self.timeout,
            paymentAmount: self.paymentAmount
        };
        self.details.insert(&(_roundId as u128), &nextDetails);
        round.startedAt = env::block_timestamp() as u64;
    }

    fn oracleInitializeNewRound(&mut self, _roundId: u64) {
        if !self.newRound(_roundId) {
            return;
        }
        let oracle_option = self.oracles.get(&env::predecessor_account_id());
        if oracle_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut oracle = oracle_option.unwrap();

        let lastStarted: u64 = oracle.lastStartedRound; // cache storage reads
        if _roundId <= lastStarted + self.restartDelay && lastStarted != 0 {
            return;
        }
        self.initializeNewRound(_roundId);
        oracle.lastStartedRound = _roundId;
        self.oracles.insert(&env::predecessor_account_id(), &oracle);
    }

    fn requesterInitializeNewRound(&mut self, _roundId: u64) {
        if !self.newRound(_roundId) {
            return;
        }
        let requester_option = self.requesters.get(&env::predecessor_account_id());
        if requester_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut requester = requester_option.unwrap();

        let lastStarted: u128 = requester.lastStartedRound.into(); // cache storage reads
        assert!((_roundId as u128) > lastStarted + (requester.delay as u128) || lastStarted == 0, "must delay requests");

        self.initializeNewRound(_roundId);

        requester.lastStartedRound = _roundId;
        self.requesters.insert(&env::predecessor_account_id(), &requester);
    }

    fn updateTimedOutRoundInfo(&mut self, _roundId: u64) {
        if !self.timedOut(_roundId) {
            return;
        }
        let prevId: u64 = _roundId - 1;

        let round_option = self.rounds.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut round = round_option.unwrap();

        let prev_option = self.rounds.get(&prevId);
        if prev_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let prev = prev_option.unwrap();

        round.answer = prev.answer;
        round.answeredInRound = prev.answeredInRound;
        round.updatedAt = env::block_timestamp() as u64;
        self.rounds.insert(&_roundId, &round);

        self.details.remove(&(_roundId as u128));
    }

    fn eligibleForSpecificRound(&self, _oracle: AccountId, _queriedRoundId: u64) -> bool {
        let round_option = self.rounds.get(&_queriedRoundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        if round.startedAt > 0 {
            return self.acceptingSubmissions(_queriedRoundId.into()) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        } else {
            return self.delayed(_oracle, _queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        }
    }

    fn oracleRoundStateSuggestRound(&mut self, _oracle: AccountId) -> (bool, u64, u128, u64, u64, u128, u64, u128) {
        let round_option = self.rounds.get(&0);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut round = round_option.unwrap();

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();

        let shouldSupersede: bool = oracle.lastReportedRound == self.reportingRoundId || !self.acceptingSubmissions(self.reportingRoundId as u128);
        // Instead of nudging oracles to submit to the next round, the inclusion of
        // the shouldSupersede bool in the if condition pushes them towards
        // submitting in a currently open round.

        let mut _roundId: u64;
        let mut _paymentAmount: u128;
        let mut _eligibleToSubmit: bool;
        let _reportingRoundId: u64 = self.reportingRoundId;

        let detail_option = self.details.get(&(_reportingRoundId as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let detail = detail_option.unwrap();

        if self.supersedable(self.reportingRoundId) && shouldSupersede {
            _roundId = self.reportingRoundId + 1;

            let roundFromId_option = self.rounds.get(&_roundId);
            if roundFromId_option.is_none() {
                env::panic(b"Did not find this round.");
            }
            let roundFromId = roundFromId_option.unwrap();
            self.rounds.insert(&0, &roundFromId);

            _paymentAmount = self.paymentAmount;
            _eligibleToSubmit = self.delayed(_oracle, _roundId);
        } else {
            let roundFromId_option = self.rounds.get(&_reportingRoundId);
            if roundFromId_option.is_none() {
                env::panic(b"Did not find this round.");
            }
            let roundFromId = roundFromId_option.unwrap();
            self.rounds.insert(&0, &roundFromId);

            _paymentAmount = detail.paymentAmount;
            _eligibleToSubmit = self.acceptingSubmissions(_roundId.into());
        }

        if self.validateOracleRound(_oracle, _roundId).len() != 0 {
            _eligibleToSubmit = false;
        }

        return (
            _eligibleToSubmit,
            _roundId,
            oracle.latestSubmission,
            round.startedAt,
            detail.timeout,
            self.recordedFunds.available,
            self.oracleCount() as u64,
            _paymentAmount
        );
    }

    fn updateRoundAnswer(&mut self, _roundId: u64) -> (bool, u128) {
        let detail_option = self.details.get(&(_roundId as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        let round_option = self.rounds.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let mut round = round_option.unwrap();

        let newAnswer: u128 = self.median(detail.submissions).into();
        round.answer = newAnswer;
        round.updatedAt = env::block_timestamp() as u64;
        round.answeredInRound = _roundId;
        self.rounds.insert(&_roundId, &round);
        self.latestRoundId = _roundId;

        return (true, newAnswer);
    }

    fn validateAnswer(&self, _roundId: u64, _newAnswer: u128) {
        let av: AccountId = self.validator; // cache storage reads
        if av == "" {
            return;
        }

        let prevRound: u64 = _roundId - 1;

        let round_option = self.rounds.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let prevAnswerRoundId: u64 = round.answeredInRound;
        let prevRoundAnswer: u128 = round.answer;
        // TRY CATCH
    }

    fn payOracle(&mut self, _roundId: u64) {
        let detail_option = self.details.get(&(_roundId as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        let oracle_option = self.oracles.get(&env::predecessor_account_id());
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();

        let payment: u128 = detail.paymentAmount;
        let mut funds: Funds = self.recordedFunds;
        funds.available = funds.available - payment;
        funds.allocated = funds.allocated - payment;
        self.recordedFunds = funds;
        oracle.withdrawable = oracle.withdrawable + payment;
        self.oracles.insert(&env::predecessor_account_id(), &oracle);
    }

    fn recordSubmission(&mut self, _submission: u128, _roundId: u128) {
        assert!(self.acceptingSubmissions(_roundId), "round not accepting submissions");

        let detail_option = self.details.get(&(_roundId as u128));
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
        self.details.insert(&(_roundId as u128), &detail);

        oracle.lastReportedRound = _roundId as u64;
        oracle.latestSubmission = _submission;
        self.oracles.insert(&env::predecessor_account_id(), &oracle);
    }

    fn deleteRoundDetails(&mut self, _roundId: u64) {
        let detail_option = self.details.get(&(_roundId as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        if (detail.submissions.len() as u64) < detail.maxSubmissions {
            return;
        }

        self.details.remove(&(_roundId as u128));
    }

    fn timedOut(&mut self, _roundId: u64) -> bool {
        let round_option = self.rounds.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();

        let detail_option = self.details.get(&(_roundId as u128));
        if detail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let detail = detail_option.unwrap();

        let startedAt: u64 = round.startedAt;
        let roundTimeout: u64 = detail.timeout;
        return startedAt > 0 && roundTimeout > 0 && ((startedAt + roundTimeout) < env::block_timestamp());
    }

    fn getStartingRound(&self, _oracle: AccountId) -> u64 {
        let currentRound: u64 = self.reportingRoundId;
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();

        if currentRound != 0 && currentRound == oracle.endingRound as u64 {
            return currentRound;
        }
        return currentRound + 1;
    }

    fn previousAndCurrentUnanswered(&self, _roundId: u64, _rrId: u64) -> bool {
        let round_option = self.rounds.get(&_rrId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        return (_roundId + 1) == _rrId && round.updatedAt == 0;
    }

    fn requiredReserve(&self, payment: u128) -> u128 {
        return payment * (self.oracleCount() * RESERVE_ROUNDS);
    }

    fn addOracle(&mut self, _oracle: AccountId, _admin: AccountId) {
        assert!(!self.oracleEnabled(_oracle), "oracle already enabled");
        assert!(_admin != "", "cannot set admin to 0");

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let mut oracle = oracle_option.unwrap();

        assert!(oracle.admin == env::predecessor_account_id() || oracle.admin == _admin, "owner cannot overwrite admin");

        oracle.startingRound = self.getStartingRound(_oracle);
        oracle.endingRound = ROUND_MAX;
        oracle.index = self.oracleAddresses.len() as u64;
        self.oracleAddresses.push(_oracle);
        oracle.admin = _admin;
        self.oracles.insert(&_oracle, &oracle);
    }

    fn removeOracle(&mut self, _oracle: AccountId) {
        assert!(self.oracleEnabled(_oracle), "oracle not enabled");

        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();

        let lastOracle: usize = (self.oracleCount() - 1).try_into().unwrap();
        let lastOracle_option = self.oracleAddresses.get(lastOracle);
        if lastOracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let lastOracle = lastOracle_option.unwrap();

        oracle.endingRound = (self.reportingRoundId + 1).into();
        let tail: AccountId = lastOracle.to_string();

        let oracleTail_option = self.oracles.get(&tail);
        if oracleTail_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracleTail = oracleTail_option.unwrap();

        let index: usize = oracle.index.try_into().unwrap();
        oracleTail.index = index.try_into().unwrap();
        oracle.index = 0_u64;
        let oracleIndex = self.oracleAddresses[index];
        oracleIndex = tail;
        self.oracleAddresses.pop();
    }

    fn validateOracleRound(&self, _oracle: AccountId, _roundId: u64) -> Base64String {
        // cache storage reads
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        let startingRound: u64 = oracle.startingRound;
        let rrId: u64 = self.reportingRoundId;

        if startingRound == 0 {
            return "not enabled oracle".to_string();
        }
        else if startingRound > _roundId {
            return "not yet enabled oracle".to_string();
        }
        else if oracle.endingRound < _roundId.into() {
            return "no longer allowed oracle".to_string();
        }
        else if oracle.lastReportedRound >= _roundId {
            return "cannot report on previous rounds".to_string();
        }
        else if _roundId != rrId && _roundId != rrId + 1 && !self.previousAndCurrentUnanswered(_roundId, rrId) {
            return "invalid round to report".to_string();
        }
        else {
            return "previous round not supersedable".to_string();
        }
    }
    fn supersedable(&mut self, _roundId: u64) -> bool {
        let round_option = self.rounds.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        round.updatedAt > 0 || self.timedOut(_roundId)
    }

    fn oracleEnabled(&self, _oracle: AccountId) -> bool {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        oracle.endingRound == ROUND_MAX
    }

    fn acceptingSubmissions(&self, _roundId: u128) -> bool {
        let round_option = self.details.get(&_roundId);
        if round_option.is_none() {
            env::panic(b"Did not find this round.");
        }
        let round = round_option.unwrap();
        round.maxSubmissions != 0
    }

    fn delayed(&self, _oracle: AccountId, _roundId: u64) -> bool {
        let oracle_option = self.oracles.get(&_oracle);
        if oracle_option.is_none() {
            env::panic(b"Did not find this oracle account.");
        }
        let oracle = oracle_option.unwrap();
        let lastStarted: u64 = oracle.lastStartedRound;
        _roundId > (lastStarted + self.restartDelay) || lastStarted == 0
    }

    fn newRound(&self, _roundId: u64) -> bool {
        _roundId == self.reportingRoundId + 1
    }

    fn validRoundId(&self, _roundId: u128) -> bool {
        _roundId <= ROUND_MAX
    }

    fn onlyOwner(&mut self) {
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
        self.description
    }
    pub fn get_version(&self) -> u128 {
        version
    }
    // Access Control

    pub fn hasAccess(&self, _user: AccountId) -> bool {
        if !self.checkEnabled {
            !self.checkEnabled
        } else {
            let user_option = self.accessList.get(&_user);
            if user_option.is_none() {
                env::panic(b"Did not find this oracle account.");
            }
            let user = user_option.unwrap();
            user
        }
    }

    pub fn addAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        let user_option = self.accessList.get(&_user);
        if user_option.is_none() {
            self.accessList.insert(&_user, &true);
            env::panic(b"Added access to this oracle account.");
        }
    }

    pub fn removeAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        let user_option = self.accessList.get(&_user);
        if user_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        self.accessList.insert(&_user, &false);
    }

    pub fn enableAccessCheck(&mut self) {
        self.onlyOwner();

        if !self.checkEnabled {
            self.checkEnabled = true;
        }
    }

    pub fn disableAccessCheck(&mut self) {
        self.onlyOwner();

        if self.checkEnabled {
            self.checkEnabled = false;
        }
    }

    fn checkAccess(&self) {
        assert!(self.hasAccess(env::predecessor_account_id()), "No access")
    }
}
