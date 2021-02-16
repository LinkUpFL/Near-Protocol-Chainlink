use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use std::str;
use num_traits::pow;

fn median(numbers: &mut [u32]) -> u32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

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
    endingRound: u64,
    lastReportedRound: u64,
    lastStartedRound: u64,
    latestSubmission: u128,
    index: u16,
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
const V3_NO_DATA_ERROR: Base64String = "No data present";

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
    details: LookupMap<u64, RoundDetails>,
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
    pub fn new(link_id: AccountId, owner_id: AccountId, _paymentAmount: U128, _timeout: U64, _validator: AccountId, _minSubmissionValue: U128, _maxSubmissionValue: U128, _decimals: U128, _description: Base64String) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(link_id.as_bytes()), "Link token account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let paymentAmount_u128: u128 = _paymentAmount.into();
        let timeout_u64: u64 = _timeout.into();
        let minSubmissionValue_u128: u128 = _minSubmissionValue.into();
        let maxSubmissionValue_u128: u128 = _maxSubmissionValue.into();
        let decimals_u128: u128 = _decimals.into();

        let mut result = Self {
            owner: owner_id,
            linkToken: link_id,
            minSubmissionValue: minSubmissionValue_u128,
            maxSubmissionValue: maxSubmissionValue_u128,
            decimals: decimals_u128,
            description: _description
        };
        result.checkEnabled = true;
        result.rounds[0].updatedAt = (env::block_timestamp() - timeout_u64) as u64;
        result.updateFutureRounds(&paymentAmount_u128, 0, 0, 0, timeout_u64);
        result.setValidator(&_validator);
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
        assert!((self.oracleCount + _added.len()) as u128 <= MAX_ORACLE_COUNT, "max oracles allowed");

        for i in 0.._added.len() {
            self.addOracle(_added[i], _addedAdmins[i]);
        }

        self.updateFutureRounds(self.paymentAmount, minSubmissions_u64, maxSubmissions_u64, restartDelay_u64, self.timeout);
    }

    pub fn updateFutureRounds(&mut self, _paymentAmount: U128, _minSubmissions: U64, _maxSubmissions: U64, _restartDelay: U64, _timeout: U64) {
        let paymentAmount_u128: u128 = _paymentAmount.into();
        let minSubmissions_u64: u64 = _minSubmissions.into();
        let maxSubmissions_u64: u64 = _maxSubmissions.into();
        let restartDelay_u64: u64 = _restartDelay.into();
        let timeout_u64: u64 = _timeout.into();

        let oracleNum: u64 = self.oracleCount(); // Save on storage reads
        assert!(maxSubmissions_u64 >= minSubmissions_u64, "max must equal/exceed min");
        assert!(oracleNum >= maxSubmissions_u64, "max cannot exceed total");
        assert!(oracleNum == 0 || oracleNum > restartDelay_u64, "delay cannot exceed total");
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

    pub fn updateAvailableFunds(&self) {
        let funds: Funds = self.recordedFunds;

        // uint256 nowAvailable = linkToken.balanceOf(address(this)).sub(funds.allocated);
        let nowAvailable: u128 = funds.available - funds.allocated;
        
        if funds.available != nowAvailable {
            self.recordedFunds.available = nowAvailable as u128;
        }
    }

    pub fn oracleCount(&self) -> u128 {
        self.oracleAddresses.len()
    }

    pub fn getOracles(&self) -> Vec<AccountId> {
        self.oracleAddresses
    }

    pub fn latestAnswer(&self) -> u128 {
        self.rounds[self.latestRoundId].answer
    }

    pub fn latestTimestamp(&self) -> u64 {
        self.rounds[self.latestRoundId].updatedAt
    }

    pub fn latestRound(&self) -> u64 {
        self.latestRoundId
    }

    pub fn getAnswer(&self, _roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();
        if self.validRoundId(_roundId) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    pub fn getTimestamp(&self, _roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();
        if self.validRoundId(_roundId) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    pub fn getRoundData(&self, _roundId: U128) -> ( u128, u128,  u64, u64, u64) {
        let roundId_u128: u128 = _roundId.into();
        let r: Round = self.rounds[roundId_u128 as u64];

        assert!(r.answeredInRound > 0 && self.validRoundId(roundId_u128), V3_NO_DATA_ERROR);

        return(
            roundId_u128,
            r.answer,
            r.startedAt,
            r.updatedAt,
            r.answeredInRound
        )
    }

    pub fn latestRoundData(&self) -> ( u128, u64, u64, u64, u128) {
        self.getRoundData(self.latestRoundId)
    }

    pub fn withdrawablePayment(&self, _oracle: AccountId) -> u128 {
        let oracles_result = self.oracles.get(&_oracle);
        if oracles_result.is_none() {
            env::panic(b"Did not find the oracle account to fulfill.");
        }
        let withdrawable_option = oracles_result.unwrap().get(&_oracle);
        if withdrawable_option.is_none() {
            env::panic(b"Did not find the withdrawable request to fulfill.");
        }
        let withdrawable = withdrawable_option.unwrap();
        withdrawable
    }

    pub fn withdrawPayment(&mut self, _oracle: AccountId, _recipient: AccountId, _amount: U128) {
        assert!(self.oracles[_oracle].admin == env::signer_account_id(), "only callable by admin");

        // Safe to downcast _amount because the total amount of LINK is less than 2^128.
        let amount_u128: u128 = _amount.into();
        let available: u128 = self.oracles[_oracle].withdrawable;
        assert!(available >= amount_u128, "insufficient withdrawable funds");

        self.oracles[_oracle].withdrawable = available - amount_u128;
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
        self.oracles[_oracle].admin
    }

    pub fn transferAdmin(&mut self, _oracle: AccountId, _newAdmin: AccountId) {
        assert!(self.oracles[_oracle].admin == env::signer_account_id(), "only callable by admin");
        self.oracles[_oracle].pendingAdmin = _newAdmin;
    }

    pub fn acceptAdmin(&mut self, _oracle: AccountId) {
        assert!(self.oracles[_oracle].pendingAdmin == env::signer_account_id(), "only callable by pending admin");
        self.oracles[_oracle].pendingAdmin = env::predecessor_account_id();
        self.oracles[_oracle].adming = env::signer_account_id(); // DOUBLE CHECK
    }

    pub fn requestNewRound(&mut self) -> u64 {
        assert!(self.requesters[env::signer_account_id()].authorized, "not authorized requester");
        let current: u64 = self.reportingRoundId;
        assert!(self.rounds[current].updatedAt > 0 || self.timedOut(current), "prev round must be supersedable");
        let newRoundId: u64 = current + 1;
        self.requesterInitializeNewRound(newRoundId);
        return newRoundId;
    }

    pub fn setRequesterPermissions(&mut self, _requester: AccountId, _authorized: bool, _delay: U64) {
        let delay_u64: u64 = _delay.into();
        if self.requester[_requester].authorized == _authorized {
            return;
        }

        if _authorized {
            self.requesters[_requester].authorized = _authorized;
            self.requesters[_requester].delay = delay_u64;
        } else {
            self.requesters[_requester].clear();
        }
    }

    pub fn onTokenTransfer(&mut self, address: AccountId, num: U128, _data: Base64String) {
        assert!(_data.len() == 0, "transfer doesn't accept calldata");
        self.updateAvailableFunds();
    }

    pub fn oracleRoundState(&self, _oracle: AccountId, _queriedRoundId: U64) -> (bool, u64, u128, u64, u64, u128, u64, u128) {
        assert!(env::predecessor_account_id() == env::sender(), "off-chain reading only");

        let queriedRoundId_u64: u64 = _queriedRoundId.into();

        if queriedRoundId_u64 > 0 {
            let round: Round = self.rounds[queriedRoundId_u64];
            let details: RoundDetails = self.details[queriedRoundId_u64];
            return (

                self.eligibleForSpecificRound(_oracle, queriedRoundId_u64),
                queriedRoundId_u64,
                self.oracles[_oracle].latestSubmission,
                self.round.startedAt,
                self.details.timeout,
                self.recordedFunds.available,
                self.oracleCount(),
                if self.round.startedAt > 0 { self.details.paymentAmount } else { self.paymentAmount }
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
        let mut round: Round = self.rounds[0];
        let mut nextDetails: RoundDetails = self.RoundDetails(
            //new int
            self.maxSubmissionCount,
            self.minSubmissionCount,
            self.timeout,
            self.paymentAmount
        );
        self.details[_roundId] = nextDetails;
        self.rounds[_roundId].startedAt = env::block_timestamp() as u64;
    }

    fn oracleInitializeNewRound(&mut self, _roundId: u64) {
        if !self.newRound(_roundId) {
            return;
        }
        let lastStarted: u64 = self.oracles[env::signer_account_id()].lastStartedRound; // cache storage reads
        if _roundId <= lastStarted + self.restartDelay && lastStarted != 0 {
            return;
        }
        self.initializeNewRound(_roundId);

        self.oracles[env::signer_account_id()].lastStartedRound = _roundId;
    }

    fn requesterInitializeNewRound(&mut self, _roundId: u64) {
        if !self.newRound(_roundId) {
            return;
        }
        let lastStarted: u128 = self.requesters[env::signer_account_id()].lastStartedRound; // cache storage reads
        assert!(_roundId > lastStarted + self.requesters[env::signer_account_id()].delay || lastStarted == 0, "must delay requests");

        self.initializeNewRound(_roundId);

        self.requesters[env::signer_account_id()].lastStartedRound = _roundId;
    }

    fn updateTimedOutRoundInfo(&mut self, _roundId: u64) {
        if !self.timedOut(_roundId) {
            return;
        }

        let prevId: u64 = _roundId - 1;
        self.rounds[_roundId].answer = self.rounds[prevId].answer;
        self.rounds[_roundId].answeredInRound = self.rounds[prevId].answeredInRound;
        self.rounds[_roundId].updatedAt = env::block_timestamp() as u64;

        self.details[_roundId].clear();
    }

    fn eligibleForSpecificRound(&self, _oracle: AccountId, _queriedRoundId: u128) -> bool {
        if self.rounds[_queriedRoundId].startedAt > 0 {
            return self.acceptingSubmissions(_queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        } else {
            return self.delayed(_oracle, _queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        }
    }

    fn oracleRoundStateSuggestRound(&mut self, _oracle: AccountId) -> ( bool,  u64,  u128, u64, u64,  u128, u128) {
        let round: Round = self.rounds[0];
        let oracle: OracleStatus = self.oracles[_oracle];

        let shouldSupersede: bool = self.oracle.lastReportedRound == self.reportingRoundId || !self.acceptingSubmissions(self.reportingRoundId as u128);
        // Instead of nudging oracles to submit to the next round, the inclusion of
        // the shouldSupersede bool in the if condition pushes them towards
        // submitting in a currently open round.

        let mut _roundId: u64;
        let mut _paymentAmount: u128;
        let mut _eligibleToSubmit: bool;

        if self.supersedable(self.reportingRoundId) && self.shouldSupersede {
            _roundId = self.reportingRoundId + 1;
            self.round = self.rounds[_roundId];

            _paymentAmount = self.paymentAmount;
            _eligibleToSubmit = self.delayed(_oracle, _roundId);
        } else {
            _roundId = self.reportingRoundId;
            self.round = self.rounds[_roundId];

            _paymentAmount = self.details[_roundId].paymentAmount;
            _eligibleToSubmit = self.acceptingSubmissions(_roundId);
        }

        if self.validateOracleRound(_oracle, _roundId).len() != 0 {
            _eligibleToSubmit = false;
        }

        return (
            _eligibleToSubmit,
            _roundId,
            self.oracle.latestSubmission,
            self.round.startedAt,
            self.details[_roundId].timeout,
            self.oracleCount(),
            _paymentAmount
        );
    }

    fn updateRoundAnswer(&mut self, _roundId: u64) -> (bool, u128) {
        // LookupMap cannot be indexed as it does not implement the index trait: https://doc.rust-lang.org/std/ops/trait.Index.html
        // let indexedRound: Option<RoundDetails> = self.details.get(&_roundId);
        // let indexedRound: RoundDetails = self.details.get(&_roundId) {
        //     Some(value) => {
        //     value;
        // }, None => RoundDetails
        // };
        // if  indexedRound < self.details[_roundId].minSubmissions {
        //     return (false, 0);
        // }
        // numbers: &mut [u32]
        let newAnswer: i128 = median(self.details[_roundId].submissions).into();

        // let newAnswer: u128 = 
        self.rounds[_roundId].answer = newAnswer;
        self.rounds[_roundId].updatedAt = env::block_timestamp() as u64;
        self.rounds[_roundId].answeredInRound = _roundId;
        self.latestRoundId = _roundId;

        return (true, newAnswer);
    }

    fn validateAnswer(&self, _roundId: u64, _newAnswer: u128) {
        let av: AccountId = self.validator; // cache storage reads
        if av == "" {
            return;
        }
        
        let prevRound: u64 = _roundId - 1;
        let prevAnswerRoundId: u64 = self.rounds[prevRound].answeredInRound;
        let prevRoundAnswer: u128 = self.rounds[prevRound].answer;
        // TRY CATCH
    }

    fn payOracle(&mut self, _roundId: u64) {
        let payment: u128 = self.details[_roundId].paymentAmount;
        let funds: Funds = self.recordedFunds;
        funds.available = funds.available - payment;
        funds.allocated = funds.allocated - payment;
        recordedFunds = funds;
        oracles[env::signer_account_id()].withdrawable = self.oracles[env::signer_account_id()].withdrawable + payment;
    }

    fn recordSubmission(&mut self, _submission: u128, _roundId: u128) {
        assert!(self.acceptingSubmissions(_roundId), "round not accepting submissions");

        self.details[_roundId].submissions.push(_submission);
        self.oracles[env::signer_account_id()].lastReportedRound = _roundId;
        self.oracles[env::signer_account_id()].latestSubmission = _submission;
    }

    fn deleteRoundDetails(&mut self, _roundId: u64) {
        if self.details[_roundId].submissions.len() < self.details[_roundId].maxSubmissions {
            return;
        }

        self.details[_roundId].clear();
    }

    fn timedOut(&mut self, _roundId: u64) -> bool {
        let startedAt: u64 = self.rounds[_roundId].startedAt;
        let roundTimeout: u64 = self.details[_roundId].timeout;
        return startedAt > 0 && roundTimeout > 0 && (startedAt + roundTimeout) < env::block_timestamp();
    }

    fn getStartingRound(&self, _oracle: AccountId) -> u64 {
        let currentRound: u64 = self.reportingRoundId;
        if currentRound != 0 && currentRound == self.oracles[_oracle].endingRound{
            return currentRound;
        }
        return currentRound + 1;
    }

    fn previousAndCurrentUnanswered(&self, _roundId: u64, _rrId: u64) -> bool {
        return (_roundId + 1) == (_rrId && self.rounds[_rrId].updatedAt == 0);
    }

    fn requiredReserve(&self, payment: u128) -> u128 {
        return payment * (self.oracleCount() * RESERVE_ROUNDS);
    }

    fn addOracle(&mut self, _oracle: AccountId, _admin: AccountId) {
        assert!(!self.oracleEnabled(_oracle), "oracle already enabled");

        assert!(_admin != "", "cannot set admin to 0");
        assert!(self.oracles[_oracle].admin == env::predecessor_account_id() || self.oracles[_oracle].admin == _admin, "owner cannot overwrite admin");

        self.oracles[_oracle].startingRound = self.getStartingRound(_oracle);
        self.oracles[_oracle].endingRound = ROUND_MAX;
        self.oracles[_oracle].index = self.oracleAddresses.len() as u16;
        self.oracleAddresses.push(_oracle);
        self.oracles[_oracle].admin = _admin;
    }

    fn removeOracle(&mut self, _oracle: AccountId) {
        assert!(self.oracleEnabled(_oracle), "oracle not enabled");

        self.oracles[_oracle].endingRound = self.reportingRoundId + 1;
        let tail: AccountId = self.oracleAddresses[self.oracleCount()-1];
        let index: u16 = self.oracles[_oracle].index;
        self.oracles[tail].index = index;
        self.oracles[_oracle].index.clear();
        self.oracleAddresses[index] = tail;
        self.oracleAddresses.pop();
    }

    fn validateOracleRound(&self, _oracle: AccountId, _roundId: u64) -> Base64String {
        // cache storage reads
        let startingRound: u64 = self.oracles[_oracle].startingRound;
        let rrId: u64 = self.reportingRoundId;

        if startingRound == 0 {
            return "not enabled oracle";
        }
        else if startingRound > _roundId {
            return "not yet enabled oracle";
        }
        else if self.oracles[_oracle].endingRound < _roundId {
            return "no longer allowed oracle";
        }
        else if self.oracles[_oracle].lastReportedRound >= _roundId {
            return "cannot report on previous rounds";
        }
        else if _roundId != rrId && _roundId != rrId + 1 && !self.previousAndCurrentUnanswered(_roundId, rrId) {
            return "invalid round to report";
        }
        else {
            return "previous round not supersedable";
        }
    }
    fn supersedable(&self, _roundId: u64) -> bool {
        self.rounds[_roundId].updatedAt > 0 || self.timedOut(_roundId)
    }

    fn oracleEnabled(&self, _oracle: AccountId) -> bool {
        self.oracles[_oracle].endingRound == ROUND_MAX
    }

    fn acceptingSubmissions(&self, _roundId: u128) -> bool {
        self.details[_roundId].maxSubmissions != 0
    }

    fn delayed(&self, _oracle: AccountId, _roundId: u64) -> bool {
        let lastStarted: u64 = self.oracles[_oracle].lastStartedRound;
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

    // Access Control

    pub fn hasAccess(&self, _user: AccountId) -> bool {
        let oracle_id_option = self.accessList.get(&_user);
        if oracle_id_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        let oracle_id = oracle_id_option.unwrap();
        self.accessList[_user] || !self.checkEnabled;
    }

    pub fn addAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        let oracle_id_option = self.accessList.get(&_user);
        if oracle_id_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        let oracle_id = oracle_id_option.unwrap();

        if !oracle_id {
            oracle_id = true;
        }
    }

    pub fn removeAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        let oracle_id_option = self.oracles.get(&_user);
        if oracle_id_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        let oracle_id = oracle_id_option.unwrap();

        if oracle_id {
            oracle_id = false;
        }
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
