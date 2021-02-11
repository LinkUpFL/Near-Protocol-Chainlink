use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use num_traits::pow;
use std::str;

pub type Base64String = String;

#[derive(Serialize, Deserialize)]
pub struct Round {
    answer: u128,
    startedAt: u64,
    updatedAt: u64,
    answeredInRound: u64
}

#[derive(Serialize, Deserialize)]
pub struct RoundDetails {
    submissions: u128[],
    maxSubmissions: u64,
    minSubmissions: u64,
    timeout: u64,
    paymentAmount: u128
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Requester {
    authorized: bool,
    delay: u64,
    lastStartedRound: u64
}

#[derive(Serialize, Deserialize)]
pub struct Funds {
    available: u128,
    allocated: u128
}

const version: u256 = 3;
const RESERVE_ROUNDS: u256 = 2;
const MAX_ORACLE_COUNT: u256 = 77;
const ROUND_MAX: u64 = 2.pow(32-1);
const V3_NO_DATA_ERROR: Base64String = "No data present";

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AccessControlledAggregator {
    pub owner: AccountId,
    pub linkToken: AccountId,
    pub validator: AccountId,
    pub paymentAmount: u128,
    pub maxSubmissionCount: u64,
    pub minSubmissionCount: u64,
    pub restartDelay: u64,
    pub timeout: u64,
    pub decimals: u8,
    pub description: Base64String,
    pub minSubmissionValue: i256,
    pub maxSubmissionValue: i256,
    pub checkEnabled: bool,
    accessList: LookupMap<AccountId, bool>,
    reportingRoundId: u64,
    latestRoundId: u64,
    oracles: LookupMap<AccountId, OracleStatus>,
    rounds: LookupMap<u64, Round>,
    details: LookupMap<u64, RoundDetails>,
    requesters: LookupMap<AccountId, Requester>,
    oracleAddresses: AccountId[],
    recordedFunds: Funds
}

impl Default for AccessControlledAggregator {
    fn default() -> Self {
        panic!("AccessControlledAggregator should be initialized before usage")
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
        let decimals_u128: u128 = _decimals.into();

        Self {
            owner: owner_id,
            linkToken: link_id,
            self.updateFutureRounds(&paymentAmount_u128, 0, 0, 0, timeout_u64);
            self.setValidator(&_validator);
            self.minSubmissionValue = minSubmissionValue_u128;
            self.maxSubmissionValue = maxSubmissionValue_u128;
            self.decimals = decimals_u128;
            self.description = _description;
            self.rounds[0].updatedAt = (env::block_timestamp - timeout_u64) as u64;
            self.checkEnabled = true;
        }
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
        let (updated: bool, newAnswer: i256) = self.updateRoundAnswer(roundId_u128 as u64);
        self.payOracle(roundId_u128 as u64);
        self.deleteRoundDetails(roundId_u128 as u64);
        if(updated){
            self.validateAnswer(roundId_u128 as u64, newAnswer);
        }
    }

    pub fn changeOracles(&mut self, _removed: AccountId[], _added: AccountId[], _addedAdmins: AccountId[], _minSubmissions: U64, _maxSubmissions: U64, _restartDelay: U64) {
        self.onlyOwner();

        let minSubmissions_u64: u64 = _minSubmissions.into();
        let maxSubmissions_u64: u64 = _maxSubmissions.into();
        let restartDelay_u64: u64 = _restartDelay.into();

        for i in 0.._removed.len() {
            self.removeOracle(_removed[i]);
        }

        assert!(_added.len() == _addedAdmins.len(), "need same oracle and admin count");
        assert!((self.oracleCount + _added.len()) as u256 <= MAX_ORACLE_COUNT, "max oracles allowed");

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
        if(self.oracleCount() > 0) {
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
        if(self.validRoundId(_roundId)) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    pub fn getTimestamp(_roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();
        if(self.validRoundId(_roundId)) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    pub fn getRoundData(&self, _roundId: U128) -> (roundId: u128, answer: u128, startedAt: u64, updatedAt: u64, answeredInRound: u64) {
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

    pub fn latestRoundData(&self) -> (roundId: u128, answer: u64, startedAt: u64, updatedAt: u64, answeredInRound: u128) {
        self.getRoundData(self.latestRoundId)
    }

    pub fn withdrawablePayment(&self, _oracle: AccountId) -> u128 {
        self.oracles[_oracle].withdrawable
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
        let available: u256 = self.recordedFunds.available as u256;
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

    pub fn requestNewRound(&mut self) -> u80 {
        assert!(self.requesters[env::signer_account_id()].authorized, "not authorized requester");
        let current: u64 = self.reportingRoundId;
        assert!(self.rounds[current].updatedAt > 0 || self.timedOut(current), "prev round must be supersedable");
        let newRoundId: u64 = current + 1;
        self.requesterInitializeNewRound(newRoundId);
        return newRoundId;
    }

    pub fn setRequesterPermissions(&mut self, _requester: AccountId, _authorized: bool, _delay: U64) {
        let delay_u64: u64 = _delay.into();
        if(self.requester[_requester].authorized == _authorized) return;

        if(_authorized) {
            self.requesters[_requester].authorized = _authorized;
            self.requesters[_requester].delay = delay_u64;
        } else {
            self.requesters[_requester].clear();
        }
    }

    // onTokenTransfer

    pub fn oracleRoundState(&self, _oracle: AccountId, _queriedRoundId: U64) -> (_eligibleToSubmit: bool, _roundId: u64, _latestSubmission: i256, _startedAt: u64, _timeout: u64, _availableFunds: u128, _oracleCount: u8, _paymentAmount: u128) {
        // require

        let queriedRoundId_u64: u64 = _queriedRoundId.into();

        if(queriedRoundId_u64 > 0) {
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

        if(previous != _newValidator) {
            self.validator = _newValidator;
        }
    }

    fn initializeNewRound(&mut self, _roundId: u64) {
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

    fn oracleInitializeNewRound(&mut self, _roundId: u64) {
        if(!self.newRound(_roundId)) return;
        let lastStarted: u256 = self.oracles[env::signer_account_id()].lastStartedRound; // cache storage reads
        if(_roundId <= lastStarted + self.restartDelay && lastStarted != 0) return;

        self.initializeNewRound(_roundId);

        self.oracles[env::signer_account_id()].lastStartedRound = _roundId;
    }

    fn requesterInitializeNewRound(&mut self, _roundId: u64) {
        if(!self.newRound(_roundId)) return;
        let lastStarted: u256 = self.requesters[env::signer_account_id()].lastStartedRound; // cache storage reads
        assert!(_roundId > lastStarted + self.requesters[env::signer_account_id()].delay || lastStarted == 0, "must delay requests");

        self.initializeNewRound(_roundId);

        self.requesters[env::signer_account_id()].lastStartedRound = _roundId;
    }

    fn updateTimedOutRoundInfo(&mut self, _roundId: u64) {
        if(!self.timedOut(_roundId)) return;

        let prevId: u64 = _roundId - 1;
        self.rounds[_roundId].answer = self.rounds[prevId].answer;
        self.rounds[_roundId].answeredInRound = self.rounds[prevId].answeredInRound;
        self.rounds[_roundId].updatedAt = env::block_timestamp() as u64;

        self.details[_roundId].clear();
    }

    fn eligibleForSpecificRound(&self, _oracle: AccountId, _queriedRoundId: u64) -> bool {
        if(self.rounds[_queriedRoundId].startedAt > 0) {
            return self.acceptingSubmissions(_queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        } else {
            return self.delayed(_oracle, _queriedRoundId) && self.validateOracleRound(_oracle, _queriedRoundId).len() == 0;
        }
    }

    fn oracleRoundStateSuggestRound(&mut self, _oracle: AccountId) -> (_eligibleToSubmit: bool, _roundId: u64, _latestSubmission: i256, _startedAt:u64, _timeout: u64, _availableFunds: u128, _oracleCount: u8, _paymentAmount: u128) {
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

    fn updateRoundAnswer(&mut self, _roundId: u64) -> (bool, i256) {
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

    fn validateAnswer(&self, _roundId: u64, _newAnswer: i256) {
        let av: AccountId = self.validator; // cache storage reads
        if(av == "") return;
        
        let prevRound: u64 = _roundId - 1;
        let prevAnswerRoundId: u64 = self.rounds[prevRound].answeredInRound;
        let prevRoundAnswer: i256 = self.rounds[prevRound].answer;
        // TRY CATCH
    }

    fn payOracle(&mut self, _roundId: u64) {
        let payment: u128 = self.details[_roundId].paymentAmount;
        let funds: Funds = self.recordedFunds;
        self.funds.available = self.funds.available - payment;
        self.funds.allocated = self.funds.allocated - payment;
        self.recordedFunds = funds;
        self.oracles[env::signer_account_id()].withdrawable = self.oracles[env::signer_account_id()].withdrawable + payment;
    }

    fn recordSubmission(&mut self, _submission: u128, _roundId: u128) {
        assert!(self.acceptingSubmissions(_roundId), "round not accepting submissions");

        self.details[_roundId].submissions.push(_submission);
        self.oracles[env::signer_account_id()].lastReportedRound = _roundId;
        self.oracles[env::signer_account_id()].latestSubmission = _submission;
    }

    fn deleteRoundDetails(&mut self, _roundId: u64) {
        if(self.details[_roundId].submissions.len() < self.details[_roundId].maxSubmissions) return;

        self.details[roundId_u64].clear();
    }

    fn timedOut(&mut self, _roundId: u64) -> bool {
        let startedAt: u64 = self.rounds[_roundId].startedAt;
        let roundTimeout: u64 = self.details[_roundId].timeout
        return(startedAt > 0 && roundTimeout > 0 && (startedAt + roundTimeout) < env::block_timestamp());
    }

    fn getStartingRound(&self, _oracle: AccountId) -> u64 {
        let currentRound: u64 = self.reportingRoundId;
        if(currentRound != 0 && currentRound == self.oracles[_oracle].endingRound){
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
        let tail: AccountId = self.oracleAddresses[self.oracleCount()-1 as u256];
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

        if (startingRound == 0) return "not enabled oracle";
        if (startingRound > _roundId) return "not yet enabled oracle";
        if (self.oracles[_oracle].endingRound < _roundId) return "no longer allowed oracle";
        if (self.oracles[_oracle].lastReportedRound >= _roundId) return "cannot report on previous rounds";
        if (_roundId != rrId && _roundId != rrId + 1 && !self.previousAndCurrentUnanswered(_roundId, rrId)) return "invalid round to report";
        if (_roundId != 1 && !self.supersedable(_roundId - 1) return "previous round not supersedable";
    }

    fn supersedable(&self, _roundId: u64) -> bool {
        self.rounds[_roundId].updatedAt > 0 || self.timedOut(_roundId)
    }

    fn oracleEnabled(&self, _oracle: AccountId) -> bool {
        self.oracles[_oracle].endingRound == ROUND_MAX
    }

    fn acceptingSubmissions(&self, _roundId: u64) -> bool {
        self.details[_roundId].maxSubmissions != 0
    }

    fn delayed(&self, _oracle: AccountId, _roundId: u64) -> bool {
        let lastStarted: u256 = self.oracles[_oracle].lastStartedRound;
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
        self.accessList[_user] || !checkEnabled;
    }

    pub fn addAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        if(!self.accessList[_user]) {
            self.accessList[_user] = true;
        }
    }

    pub fn removeAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        if(self.accessList[_user]) {
            self.accessList[_user] = false;
        }
    }

    pub fn enableAccessCheck(&mut self) {
        self.onlyOwner();

        if(!self.checkEnabled) {
            self.checkEnabled = true;
        }
    }

    pub fn disableAccessCheck(&mut self) {
        self.onlyOwner();

        if(self.checkEnabled) {
            self.checkEnabled = false;
        }
    }

    fn checkAccess(&self) {
        assert!(self.hasAccess(env::predecessor_account_id()), "No access")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Necessary NEAR accounts for testing data flow
    fn accessControlledAggregator() -> AccountId { "access_controlled_aggregator_near".to_string() }
    fn nolanNear() -> AccountId { "nolan_near".to_string() }
    fn linkToken() -> AccountId { "link_token_near".to_string() }
    fn validator() -> AccountId { "validator_near".to_string() }
    fn oracleOne() -> AccountId {"oracle_one".to_string()}
    fn oracleTwo() -> AccountId {"oracle_two".to_string()}
    fn oracleThree() -> AccountId {"oracle_three".to_string()}

    // VM Context sends mock transactions with the context below, 
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: nolanNear(),
            signer_account_id,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: nolanNear(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            epoch_height: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn initialize_contract() {
        let context = get_context(accessControlledAggregator(), 0);
        testing_env!(context);
        let mut contract = AccessControlledAggregator::new(link(), nolanNear(), 1, 100, validator(), 0, 1000, 4, "a Description");
        assert_eq!(contract.onlyOwner(), true);
    } 
}