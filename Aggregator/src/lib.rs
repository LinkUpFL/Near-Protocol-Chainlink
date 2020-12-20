use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet, LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Answer {
    minimumResponses: u128,
    maxResponses: u128,
    responses: i256[]
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Aggregator {
    currentAnswerValue: i256,
    updatedTimestampValue: u256,
    latestCompletedAnswer: u256,
    paymentAmount: u128,
    minimumResponses: u128,
    jobIds [u8; 4],
    oracles: AccountId[],
    answerCounter: u256,
    authorizedRequesters: LookupMap<AccountId, bool>,
    requestAnswers: LookupMap<[u8; 4], u256>,
    answers: LookupMap<u256, Answer>,
    currentAnswers: LookupMap<u256, i256>,
    updatedTimestamps: LookupMap<u256, u256>,
    MAX_ORACLE_COUNT: u256
}

#[near_bindgen]
impl Aggregator {
    pub fn requestRateUpdate(&mut self) {
        let requestId: [u8; 4];
        let oraclePayment: u256 = self.paymentAmount;
        // add more
    }

    pub fn updateRequestDetails(&mut self, _paymentAmount: u128, _minimumResponses: u128, _oracles: AccountId[], /* jobIds */) {
        self.paymentAmount = _paymentAmount;
        self.minimumResponses = _minimumResponses;
        self.jobIds = _jobIds;
        self.oracles = _oracles;
    }

    // pub fn transferLINK
    
    pub fn setAuthorization(&mut self, _requester: AccountId, _allowed: bool) {
        self.authorizedRequesters[_requester] = _allowed;
    }

    pub fn cancelRequest(&mut self, /* requestId */, _payment: u256, _expiration: u256) {
        let answerId: u256 = self.requestAnswers[_requestId];
        assert!(answerId < latestCompletedAnswer, "Cannot modify an in-progress answer");

        self.requestAnswers[_requestId].clear();
        self.answers[answerId].responses.push(0);
        self.deleteAnswer(answerId);

        // cancelChainlinkRequest
    }

    // pub fn destroy

    fn updateLatestAnswer(&mut self, _answerId: u256) {
        let responseLength: u256 = self.answers[_answerId].responses.len();
        let middleIndex: u256 = responseLength / 2;
        let currentAnswerTemp: i256;
        // add if
        self.currentAnswerValue = currentAnswerTemp;
        self.latestCompletedAnswer = _answerId;
        // add now values
        self.currentAnswers[_answerId] = currentAnswerTemp;
    }

    pub fn latestAnswer(&self) -> i256 {
        self.currentAnswers[latestCompletedAnswer]
    }

    pub fn latestTimestamp(&self) -> u256 {
        self.updatedTimestamps[latestCompletedAnswer]
    }

    pub fn getAnswer(&self, _roundId: u256) -> i256 {
        self.currentAnswers[_roundId]
    }

    pub fn getTimestamp(&self, _roundId: u256) -> u256 {
        updatedTimestamps[_roundId]
    }

    pub fn latestRound(&self) -> u256 {
        self.latestCompletedAnswer
    }

    fn quickselect(&self, _a: i256[], _k: u256) -> i256 {
        let a: i256[] = _a;
        let k: u256 = _k;
        let aLen: u256 = a.len();
        // add a1 and a2
        let a1Len: u256;
        let a2Len: u256;
        let pivot: i256;
        let i: u256;
    }

    pub fn swap(&self, _a: i256[], _b: i256[]) -> (i256[], i256[]) {
        return (_b, _a);
    }

    pub fn deleteAnswer(&mut self, _answerId: u256) {
        // assert all responses received
        self.answers[_answerId].clear();
    }

    pub fn ensureMinResponsesReceived(mut &self, _answerId: u256) {
        assert!(self.answers[_answerId].responses.len() >= self.answers[_answerId].minimumResponses), "Min Responses not yet received");
    }

    pub fn ensureAllResponsesReceived(mut &self, _answerId: u256) {
        assert!(self.answers[_answerId].responses.len() == self.answers[_answerId].maxResponses), "All Responses not yet received");
    }

    pub fn ensureOnlyLatestAnswer(mut &self, _answerId: u256) {
        assert!(self.latestCompletedAnswer <= _answerId), "Not latest answer");
    }

    pub fn validateAnswerRequirements(mut &self, _minimumResponses: u256, _oracles: AccountId[], _jobIds: [u8; 4]) {
        assert!(_oracles.len() <= self.MAX_ORACLE_COUNT, "Cannot have more than {} oracles", self.MAX_ORACLE_COUNT);
        assert!(_oracles.len() >= _minimumResponses, "must have at least as many oracles as responses");
        assert!(_oracles.len() == _jobIds.len(), "must have at least as many oracles as responses");
    }

    pub fn ensureAuthorizedRequester(mut &self) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Not an authorized address for creating requests");
    }
}
