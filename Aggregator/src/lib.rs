use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet, LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

const MAX_ORACLE_COUNT: u256 = 28;

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
    jobIds: Base64String[],
    oracles: AccountId[],
    answerCounter: u256,
    authorizedRequesters: LookupMap<AccountId, bool>,
    requestAnswers: LookupMap<Base64String, u256>,
    answers: LookupMap<u256, Answer>,
    currentAnswers: LookupMap<u256, i256>,
    updatedTimestamps: LookupMap<u256, u256>,
}

#[near_bindgen]
impl Aggregator {
    pub fn requestRateUpdate(&mut self) {
        self.ensureAuthorizedRequester();
        let requestId: Base64String;
        let oraclePayment: u256 = self.paymentAmount;
        // for loop (build chainlink request??)
        self.answers[self.answerCounter].minimumResponses = self.minimumResponses;
        self.answers[self.answerCounter].maxResponses = self.oracles.len() as u128;

        self.answerCounter = self.answerCounter + 1;
    }

    pub fn chainlinkCallback(&mut self, _clRequestId: Base64String, _response: i256) {
        // validate ??

        let answerId: u256 = self.requestAnswers(_clRequestId);
        self.requestAnswers[_clRequestId].clear();

        self.answers[answerId].responses.push(_response);
        self.updateLatestAnswer(answerId);
        self.deleteAnswer(answerId);
    }

    pub fn updateRequestDetails(&mut self, _paymentAmount: u128, _minimumResponses: u128, _oracles: AccountId[], _jobIds: Base64String[]) {
        self.onlyOwner();
        self.validateAnswerRequirements(_minimumResponses, _oracles, _jobIds);
        self.paymentAmount = _paymentAmount;
        self.minimumResponses = _minimumResponses;
        self.jobIds = _jobIds;
        self.oracles = _oracles;
    }

    // pub fn transferLINK
    
    pub fn setAuthorization(&mut self, _requester: AccountId, _allowed: bool) {
        self.onlyOwner();
        self.authorizedRequesters[_requester] = _allowed;
    }

    pub fn cancelRequest(&mut self, _requestId: Base64String, _payment: u256, _expiration: u256) {
        self.ensureAuthorizedRequester();
        let answerId: u256 = self.requestAnswers[_requestId];
        assert!(answerId < latestCompletedAnswer, "Cannot modify an in-progress answer");

        self.requestAnswers[_requestId].clear();
        self.answers[answerId].responses.push(0);
        self.deleteAnswer(answerId);

        // cancelChainlinkRequest
    }

    // pub fn destroy

    fn updateLatestAnswer(&mut self, _answerId: u256) {
        self.ensureMinResponsesReceived(_answerId);
        self.ensureOnlyLatestAnswer(_answerId);
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
        let mut a: i256[] = _a;
        let mut k: u256 = _k;
        let mut aLen: u256 = a.len();
        // add a1 and a2
        let mut a1Len: u256;
        let mut a2Len: u256;
        let mut pivot: i256;

        while(true) {
            pivot = a[aLen / 2];
            a1Len = 0;
            a2Len = 0;
            for i in 0..aLen {
                if(a[i] < pivot) {
                    a1[a1Len] = a[i];
                    a1Len = a1Len + 1;
                } else if =(a[i] > pivot) {
                    a2[a2Len] = a[i];
                    a2Len = a2Len + 1;
                }
            }
            if(k <= a1Len) {
                aLen = a1Len;
                a, a1 = self.swap(a, a1); // CHECK
            } else if(k > (aLen - a2Len)) {
                k = k - (aLen - a2Len);
                aLen = a2Len;
                a, a2 = self.swap(a, a2); // CHECK
            } else {
                return pivot;
            }
        }
    }

    fn swap(&self, _a: i256[], _b: i256[]) -> (i256[], i256[]) {
        return (_b, _a);
    }

    fn deleteAnswer(&mut self, _answerId: u256) {
        self.ensureAllResponsesReceived(_answerId);
        self.answers[_answerId].clear();
    }

    fn ensureMinResponsesReceived(mut &self, _answerId: u256) {
        assert!(self.answers[_answerId].responses.len() >= self.answers[_answerId].minimumResponses), "Min Responses not yet received");
    }

    fn ensureAllResponsesReceived(mut &self, _answerId: u256) {
        assert!(self.answers[_answerId].responses.len() == self.answers[_answerId].maxResponses), "All Responses not yet received");
    }

    fn ensureOnlyLatestAnswer(mut &self, _answerId: u256) {
        assert!(self.latestCompletedAnswer <= _answerId), "Not latest answer");
    }

    fn validateAnswerRequirements(mut &self, _minimumResponses: u256, _oracles: AccountId[], _jobIds: Base64String[]) {
        assert!(_oracles.len() <= MAX_ORACLE_COUNT, "Cannot have more than {} oracles", MAX_ORACLE_COUNT);
        assert!(_oracles.len() >= _minimumResponses, "must have at least as many oracles as responses");
        assert!(_oracles.len() == _jobIds.len(), "must have at least as many oracles as responses");
    }

    fn ensureAuthorizedRequester(mut &self) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Not an authorized address for creating requests");
    }

    fn onlyOwner(&mut self) {
        assert_eq!(env::signer_account_id(), env::current_account_id(), "Only contract owner can call this method.");
    }
}
