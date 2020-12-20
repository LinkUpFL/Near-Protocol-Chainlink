use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

pub type Base64String = String;

#[derive(Serialize, Deserialize)]
pub struct Round {
    answer: i256,
    startedAt: u64,
    updatedAt: u64,
    answeredInRound: u32
}

#[derive(Serialize, Deserialize)]
pub struct RoundDetails {
    submissions: i256[],
    maxSubmissions: u32,
    minSubmissions: u32,
    timeout: u32,
    paymentAmount: u128
}

#[derive(Serialize, Deserialize)]
pub struct OracleStatus {
    withdrawable: u128,
    startingRound: u32,
    endingRound: u32,
    lastReportedRound: u32,
    lastStartedRound: u32,
    latestSubmission: i256,
    index: u16,
    admin: AccountId,
    pendingAdmin: AccountId
}

#[derive(Serialize, Deserialize)]
pub struct Requester {
    authorized: bool,
    delay: u32,
    lastStartedRound: u32
}

#[derive(Serialize, Deserialize)]
pub struct Funds {
    available: u128,
    allocated: u128
}

const version: u256 = 3;
const RESERVE_ROUNDS: u256 = 2;
const MAX_ORACLE_COUNT: u256 = 77;
const ROUND_MAX: u32 = 2.pow(32-1);
const V3_NO_DATA_ERROR: Base64String = "No data present";

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct FluxAggregator {
    pub linkToken: AccountId,
    pub validator: AccountId,
    pub paymentAmount: u128,
    pub maxSubmissionCount: u32,
    pub minSubmissionCount: u32,
    pub restartDelay: u32,
    pub timeout: u32,
    pub decimals: u8,
    pub description: Base64String,
    pub minSubmissionValue: i256,
    pub maxSubmissionValue: i256,
    reportingRoundId: u32,
    latestRoundId: u32,
    oracles: LookupMap<AccountId, OracleStatusc>,
    rounds: LookupMap<u32, Round>,
    details: LookupMap<u32, RoundDetails>,
    requesters: LookupMap<AccountId, Requester>,
    oracleAddresses: AccountId[],
    recordedFunds: Funds
}

#[near_bindgen]
impl FluxAggregator {
    pub fn submit(&mut self, _roundId: u256, _submission: i256) {
        // assert

        self.oracleInitializeNewRound(_roundId as u32);
        self.recordSubmission(_submission, _roundId as u32);
        // update Answer
        self.payOracle(_roundId as u32);
        self.deleteRoundDetails(_roundId as u32);
        // if for updated
    }

    pub fn changeOracles(&mut self, _removed: AccountId[], _added: AccountId[], _addedAdmins: AccountId[], _minSubmissions: u32, _maxSubmissions: u32, _restartDelay: u32) {
        for i in 0.._removed.len() {
            self.removeOracle(_removed[i]);
        }

        assert!(_added.len() == _addedAdmins.len(), "need same oracle and admin count");
        assert!((self.oracleCount + _added.len()) as u256 <= self.MAX_ORACLE_COUNT, "max oracles allowed");

        for i in 0.._added.len() {
            self.addOracle(_added[i], _addedAdmins[i]);
        }

        self.updateFutureRounds(self.paymentAmount, _minSubmissions, _maxSubmissions, _restartDelay, self.timeout);
    }

    pub fn updateFutureRounds(&mut self, _paymentAmount: u128, _minSubmissions: u32, _maxSubmissions: u32, _restartDelay: u32, _timeout: u32) {
        let oracleNum: u32 = self.oracleCount(); // Save on storage reads
        assert!(_maxSubmissions >= _minSubmissions, "max must equal/exceed min");
        assert!(oracleNum >= _maxSubmissions, "max cannot exceed total");
        assert!(oracleNum == 0 || oracleNum > _restartDelay, "delay cannot exceed total");
        assert!(self.recordedFunds.available >= self.requiredReserve(_paymentAmount), "insufficient funds for payment");
        if(self.oracleCount() > 0) {
            assert!(_minSubmissions > 0, "min must be greater than 0")
        }

        self.paymentAmount = _paymentAmount;
        self.minSubmissionCount = _minSubmissions;
        self.maxSubmissionCount = _maxSubmissions;
        self.restartDelay = _restartDelay;
        self.timeout = _timeout;
    }

    pub fn allocatedFunds(&self) -> u128 {
        self.recordedFunds.allocated
    }

    pub fn availableFunds(&self) -> u128 {
        self.recordedFunds.available
    }

    pub fn updateAvailableFunds(&self) {
        let funds: Funds = self.recordedFunds;

        // nowAavilable

        if(funds.available != nowAavilable) {
            self.recordedFunds.available = nowAavilable as u128;
        }
    }

    pub fn oracleCount(&self) -> u8 {
        self.oracleAddresses.len() as u8
    }

    pub fn getOracles(&self) -> AccountId[] {
        self.oracleAddresses
    }

    pub fn latestAnswer(&self) -> i256 {
        self.rounds[self.latestRoundId].answer
    }

    pub fn latestTimestamp(&self) -> u256 {
        self.rounds[self.latestRoundId].updatedAt
    }

    pub fn latestRound(&self) -> u256 {
        self.latestRoundId
    }

    pub fn getAnswer(&self, _roundId: u256) -> i256 {
        if(self.validRoundId(_roundId)) {
            return self.rounds[_roundId as u32].answer;
        }
        return 0;
    }

    pub fn getTimestamp(_roundId: u256) -> u256 {
        if(self.validRoundId(_roundId)) {
            return self.rounds[_roundId as u32].answer;
        }
        return 0;
    }

    pub fn getRoundData(&self, _roundId: u80) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        let r: Round = self.rounds[_roundId as u32];

        assert!(r.answeredInRound > 0 && self.validRoundId(_roundId), V3_NO_DATA_ERROR);

        return(
            _roundId,
            r.answer,
            r.startedAt,
            r.updatedAt,
            r.answeredInRound
        )
    }

    pub fn latestRoundData(&self) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        self.getRoundData(self.latestRoundId)
    }

    pub fn withdrawablePayment(&self, _oracle: AccountId) -> u256 {
        self.oracles[_oracle].withdrawable
    }

    pub fn withdrawPayment(&mut self, _oracle: AccountId, _recipient: AccountId, _amount: u256) {
        assert!(self.oracles[_oracle].admin == env::signer_account_id(), "only callable by admin");

        // Safe to downcast _amount because the total amount of LINK is less than 2^128.
        let amount: u128 = _amount as u128;
        let available: u128 = self.oracles[_oracle].withdrawable;
        assert!(available >= amount, "insufficient withdrawable funds");

        self.oracles[_oracle].withdrawable = available - amount;
        self.recordedFunds.allocated = self.recordedFunds.allocated - amount;

        //assert(linkToken.transfer(_recipient, uint256(amount)));
    }

    pub fn withdrawFunds(&mut self, _recipient: AccountId, _amount: u256) {
        let available: u256 = self.recordedFunds.available as u256;
        assert!((available - self.requiredReserve(self.paymentAmount)) >= _amount, "insufficient reserve funds");
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

    pub fn requestNewRound(&mut self) -> u80 {
        assert!(self.requesters[env::signer_account_id()].authorized, "not authorized requester");
        let current: u32 = self.reportingRoundId;
        assert!(self.rounds[current].updatedAt > 0 || self.timedOut(current), "prev round must be supersedable");
        let newRoundId: u32 = current + 1;
        self.requesterInitializeNewRound(newRoundId);
        return newRoundId;
    }

    pub fn setRequesterPermissions(&mut self, _requester: AccountId, _authorized: bool, _delay: u32) {
        if(self.requester[_requester].authorized == _authorized) return;

        if(_authorized) {
            self.requesters[_requester].authorized = _authorized;
            self.requesters[_requester].delay = _delay;
        } else {
            self.requesters[_requester].clear();
        }
    }

    // onTokenTransfer

    //pub fn oracleRoundState(&self )

    pub fn setValidator(&mut self, _newValidator: AccountId) {
        let previous: AccountId = self.validator as AccountId;

        if(previous != _newValidator) {
            self.validator = _newValidator;
        }
    }

    fn initializeNewRound(&mut self, _roundId: u32) {
        self.updateTimedOutRoundInfo(_roundId - 1);
        self.reportingRoundId = _roundId;
        let nextDetails: self.RoundDetails = self.RoundDetails(
            //new int
            self.maxSubmissionCount,
            self.minSubmissionCount,
            self.timeout,
            self.paymentAmount
        );
        self.details[_roundId] = nextDetails;
        self.rounds[_roundId].startedAt = env::block_timestamp() as u64;
    }

    fn oracleInitializeNewRound(&mut self, _roundId: u32) {
        if(!self.newRound(_roundId)) return;
        let lastStarted: u256 = self.oracles[env::signer_account_id()].lastStartedRound; // cache storage reads
        if(_roundId <= lastStarted + self.restartDelay && lastStarted != 0) return;

        self.initializeNewRound(_roundId);

        self.oracles[env::signer_account_id()].lastStartedRound = _roundId;
    }

    fn requesterInitializeNewRound(&mut self, _roundId: u32) {
        if(!self.newRound(_roundId)) return;
        let lastStarted: u256 = self.requesters[env::signer_account_id()].lastStartedRound; // cache storage reads
        assert!(_roundId > lastStarted + self.requesters[env::signer_account_id()].delay || lastStarted == 0, "must delay requests");

        self.initializeNewRound(_roundId);

        self.requesters[env::signer_account_id()].lastStartedRound = _roundId;
    }

    fn updateTimedOutRoundInfo(&mut self, _roundId: u32) {
        if(!self.timedOut(_roundId)) return;

        let prevId: u32 = _roundId - 1;
        self.rounds[_roundId].answer = self.rounds[prevId].answer;
        self.rounds[_roundId].answeredInRound = self.rounds[prevId].answeredInRound;
        self.rounds[_roundId].updatedAt = env::block_timestamp() as u64;

        self.details[_roundId].clear();
    }

    fn eligibleForSpecificRound(&self, _oracle: AccountId, _queriedRoundId: u32) -> bool {
        if(self.rounds[_queriedRoundId].startedAt > 0) {
            return self.acceptingSubmissions(_queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        } else {
            return self.delayed(_oracle, _queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        }
    }

    fn oracleRoundStateSuggestRound(&mut self, _oracle: AccountId) -> (_eligibleToSubmit: bool, _roundId: u32, _latestSubmission: i256, _startedAt:u64, _timeout: u64, _availableFunds: u128, _oracleCount: u8, _paymentAmount: u128) {
        let round: Round = self.rounds[0];
        let oracle: OracleStatus = self.oracles[_oracle];

        let shouldSupersede: bool = self.oracle.lastReportedRound == self.reportingRoundId || !self.acceptingSubmissions(self.reportingRoundId);
        // Instead of nudging oracles to submit to the next round, the inclusion of
        // the shouldSupersede bool in the if condition pushes them towards
        // submitting in a currently open round.
        if(self.supersedable(self.reportingRoundId) && self.shouldSupersede) {
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

        if(self.validateOracleRound(_oracle, _roundId).len() != 0) {
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

    fn updateRoundAnswer(&mut self, _roundId: u32) -> (bool, i256) {
        if(self.details[_roundId].submissions.len() < self.details[_roundId].minSubmissions){
            return (false, 0);
        }

        // let newAnswer: i256 = 
        self.rounds[_roundId].answer = newAnswer;
        self.rounds[_roundId].updatedAt = env::block_timestamp() as u64;
        rounds[_roundId].answeredInRound = _roundId;
        self.latestRoundId = _roundId;

        return (true, newAnswer);
    }

    //fn validateAnswer()

    fn payOracle(&mut self, _roundId: u32) {
        let payment: u128 = self.details[_roundId].paymentAmount;
        let funds: Funds = self.recordedFunds;
        self.funds.available = self.funds.available - payment;
        self.funds.allocated = self.funds.allocated - payment;
        self.recordedFunds = funds;
        self.oracles[env::signer_account_id()].withdrawable = self.oracles[env::signer_account_id()].withdrawable + payment;
    }

    fn recordSubmission(&mut self, _submission: i256, _roundId: u32) {
        assert!(self.acceptingSubmissions(_roundId), "round not accepting submissions");

        self.details[_roundId].submissions.push(_submission);
        self.oracles[env::signer_account_id()].lastReportedRound = _roundId;
        self.oracles[env::signer_account_id()].latestSubmission = _submission;
    }

    fn deleteRoundDetails(&mut self, _roundId: u32) {
        if(self.details[_roundId].submissions.len() < self.details[_roundId].maxSubmissions) return;

        self.details[_roundId].clear();
    }

    fn timedOut(&mut self, _roundId: u32) -> bool {
        let startedAt: u64 = self.rounds[_roundId].startedAt;
        let roundTimeout: u32 = self.details[_roundId].timeout
        return(startedAt > 0 && roundTimeout > 0 && (startedAt + roundTimeout) < env::block_timestamp());
    }

    fn getStartingRound(&self, _oracle: AccountId) -> u32 {
        let currentRound: u32 = self.reportingRoundId;
        if(currentRound != 0 && currentRound == self.oracles[_oracle].endingRound){
            return currentRound;
        }
        return currentRound + 1;
    }

    fn previousAndCurrentUnanswered(&self, _roundId: u32, _rrId: u32) -> bool {
        return (_roundId + 1) == (_rrId && self.rounds[_rrId].updatedAt == 0);
    }

    fn requiredReserve(&self, payment: u256) -> u256 {
        return payment * (self.oracleCount() * self.RESERVE_ROUNDS);
    }

    fn addOracle(&mut self, _oracle: AccountId, _admin: AccountId) {
        assert!(!self.oracleEnabled(_oracle), "oracle already enabled");

        assert!(_admin != env::predecessor_account_id(), "cannot set admin to 0"); // double check address(0)
        assert!(self.oracles[_oracle].admin == env::predecessor_account_id() || self.oracles[_oracle].admin == _admin, "owner cannot overwrite admin");

        self.oracles[_oracle].startingRound = self.getStartingRound(_oracle);
        self.oracles[_oracle].endingRound = self.ROUND_MAX;
        self.oracles[_oracle].index = self.oracleAddresses.len() as u16;
        self.oracleAddresses.push(_oracle);
        self.oracles[_oracle].admin = _admin;
    }

    fn removeOracle(&mut self, _oracle: AccountId) {
        assert!(self.oracleEnabled(_oracle), "oracle not enabled");

        self.oracles[_oracle].endingRound = self.reportingRoundId + 1;
        let tail: AccountId = self.oracleAddresses[self.oracleCount()-1 as u256];
        let index: u16 = self.oracles[_oracle].index;
        self.oracles[tail].index = index;
        self.oracles[_oracle].index.clear();
        self.oracleAddresses[index] = tail;
        self.oracleAddresses.pop();
    }

    fn validateOracleRound(&self, _oracle: AccountId, _roundId: u32) -> Base64String {
        // cache storage reads
        let startingRound: u32 = self.oracles[_oracle].startingRound;
        let rrId: u32 = self.reportingRoundId;

        if (startingRound == 0) return "not enabled oracle";
        if (startingRound > _roundId) return "not yet enabled oracle";
        if (self.oracles[_oracle].endingRound < _roundId) return "no longer allowed oracle";
        if (self.oracles[_oracle].lastReportedRound >= _roundId) return "cannot report on previous rounds";
        if (_roundId != rrId && _roundId != rrId + 1 && !self.previousAndCurrentUnanswered(_roundId, rrId)) return "invalid round to report";
        if (_roundId != 1 && !self.supersedable(_roundId - 1) return "previous round not supersedable";
    }

    fn supersedable(&self, _roundId: u32) -> bool {
        self.rounds[_roundId].updatedAt > 0 || self.timedOut(_roundId)
    }

    fn oracleEnabled(&self, _oracle: AccountId) -> bool {
        self.oracles[_oracle].endingRound == self.ROUND_MAX
    }

    fn acceptingSubmissions(&self, _roundId: u32) -> bool {
        self.details[_roundId].maxSubmissions != 0
    }

    fn delayed(&self, _oracle: AccountId, _roundId: u32) -> bool {
        let lastStarted: u256 = self.oracles[_oracle].lastStartedRound;
        _roundId > (lastStarted + self.restartDelay) || lastStarted == 0
    }

    fn newRound(&self, _roundId: u32) -> bool {
        _roundId == self.reportingRoundId + 1
    }

    fn validRoundId(&self, _roundId: u256) -> bool {
        _roundId <= self.ROUND_MAX
    }
}
