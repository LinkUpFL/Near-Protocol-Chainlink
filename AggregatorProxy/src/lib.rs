use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet, LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Phase {
    id: u16,
    aggregator: AccountId
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AggregatorProxy {
    currentPhase: Phase,
    pub proposedAggregator: AccountId,
    pub phaseAggregators: LookupMap<u16, AccountId>,
    PHASE_OFFSET: u256 = 64,
    PHASE_SIZE: u256 = 16,
    // MAX_ID
}

#[near_bindgen]
impl AggregatorProxy {
    pub fn latestAnswer(&self) -> i256 {
        self.currentPhase.aggregator.latestAnswer()
    }

    pub fn latestTimestamp(&self) -> u256 {
        self.currentPhase.aggregator.latestTimestamp()
    }

    pub fn getAnswer(&mut self, _roundId: u256) -> i256 {
        if(_roundId > self.MAX_ID) return 0;
        // add more
    }

    // getTimestamp

    pub fn latestRound(&mut self) -> u256 {
        let phase: Phase = self.currentPhase;
        self.addPhase(phase.id, phase.aggregator.latestRound() as u64);
    }


}
