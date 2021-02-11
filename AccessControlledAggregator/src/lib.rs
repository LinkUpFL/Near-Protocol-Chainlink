use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;

pub type Base64String = String;

#[derive(Serialize, Deserialize)]
pub struct Round {
    answer: u128,
    startedAt: u64,
    updatedAt: u64,
    answeredInRound: u64
}

const V3_NO_DATA_ERROR: Base64String = "No data present";
const ROUND_MAX: u64 = 2.pow(32-1);

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AccessControlledAggregator {
    latestRoundId: u64,
    rounds: LookupMap<u64, Round>
}

#[near_bindgen]
impl AccessControlledAggregator {
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

    pub fn latestAnswer(&self) -> u128 {
        self.rounds[self.latestRoundId].answer
    }

    pub fn latestRound(&self) -> u64 {
        self.latestRoundId
    }

    pub fn latestTimestamp(&self) -> u64 {
        self.rounds[self.latestRoundId].updatedAt
    }

    pub fn getAnswer(&self, _roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();
        if(self.validRoundId(_roundId)) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    pub fn getTimestamp(&self, _roundId: U128) -> u128 {
        let roundId_u128: u128 = _roundId.into();
        if(self.validRoundId(_roundId)) {
            return self.rounds[roundId_u128 as u64].answer;
        }
        return 0;
    }

    // Internal

    fn validRoundId(&self, _roundId: U128) -> bool {
        let roundId_u128: u128 = _roundId.into();
        _roundId <= ROUND_MAX
    }
}
