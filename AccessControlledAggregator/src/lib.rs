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
    answeredInRound: u64
}

#[derive(Serialize, Deserialize)]
pub struct RoundDetails {
    submissions: i256[],
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
    latestSubmission: i256,
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
    reportingRoundId: u64,
    latestRoundId: u64,
    oracles: LookupMap<AccountId, OracleStatus>,
    rounds: LookupMap<u64, Round>,
    details: LookupMap<u64, RoundDetails>,
    requesters: LookupMap<AccountId, Requester>,
    oracleAddresses: AccountId[],
    recordedFunds: Funds
}

#[near_bindgen]
impl AccessControlledAggregator {
    pub fn getRoundData(&self, _roundId: U128) -> (roundId: u128, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
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

    pub fn latestRoundData(&self) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        self.getRoundData(self.latestRoundId)
    }

    pub fn latestAnswer(&self) -> i256 {
        self.rounds[self.latestRoundId].answer
    }

    pub fn latestRound(&self) -> u256 {
        self.latestRoundId
    }

    pub fn latestTimestamp(&self) -> u256 {
        self.rounds[self.latestRoundId].updatedAt
    }


    pub fn getAnswer(&self, _roundId: U128) -> i256 {
        let roundId_u128: u128 = _roundId.into();
        if(self.validRoundId(_roundId)) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    pub fn getTimestamp(_roundId: U128) -> u256 {
        let roundId_u128: u128 = _roundId.into();
        if(self.validRoundId(_roundId)) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

}
