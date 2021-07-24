use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen};
use std::str;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13

pub type Base64String = String;

const VERSION: u128 = 0;
const RESERVE_ROUNDS: u128 = 2;
const MAX_ORACLE_COUNT: u128 = 77;
const ROUND_MAX: u128 = 4294967295; // 2**32-1
const V3_NO_DATA_ERROR: &str = "No data present";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MockV3Aggregator {
    pub decimals: u128,
    pub latest_answer: u128,
    pub latest_timestamp: u128,
    pub latest_round: u128,
    pub get_answer: LookupMap<u128, u128>,
    pub get_timestamp: LookupMap<u128, u128>,
    get_started_at: LookupMap<u128, u128>,
}

impl Default for MockV3Aggregator {
    fn default() -> Self {
        panic!("MockV3Aggregator should be initialized before usage");
    }
}
/**
 * @title MockV3Aggregator
 * @notice Based on the FluxAggregator contract
 * @notice Use this contract when you need to test
 * other contract's ability to read data from an
 * aggregator contract, but how the aggregator got
 * its answer is unimportant
 */
#[near_bindgen]
impl MockV3Aggregator {
    #[init]
    pub fn new(_decimals: U128, _initial_answer: U128) -> Self {
        let mut result = Self {
            decimals: _decimals.into(),
            latest_answer: 0,
            latest_timestamp: 0,
            latest_round: 0,
            get_answer: LookupMap::new(b"get_answer".to_vec()),
            get_timestamp: LookupMap::new(b"get_timestamp".to_vec()),
            get_started_at: LookupMap::new(b"get_started_at".to_vec()),
        };
        result.update_answer(_initial_answer);
        result
    }

    pub fn update_answer(&mut self, _answer: U128) {
        let new_round: u128 = self.latest_round + 1;
        let current_timestamp: u64 = env::block_timestamp();
        self.latest_answer = _answer.into();
        self.latest_timestamp = u128::from(current_timestamp);
        self.latest_round = new_round;

        let get_answer = self.get_answer.get(&new_round);
        if get_answer.is_none() {
            self.get_answer.insert(&new_round, _answer);
        } else {
            let mut get_answer_update = get_answer.unwrap();
            self.get_answer.remove(&new_round);
            self.get_answer.insert(&new_round, _answer);
        }

        let get_timestamp = self.get_timestamp.get(&new_round);
        if get_timestamp.is_none() {
            self.get_timestamp
                .insert(&new_round, env::block_timestamp());
        } else {
            let mut get_timestamp_update = get_timestamp.unwrap();
            self.get_timestamp.remove(&new_round);
            self.get_timestamp
                .insert(&new_round, env::block_timestamp());
        }

        let get_started_at = self.get_started_at.get(&new_round);
        if get_started_at.is_none() {
            self.get_started_at
                .insert(&new_round, env::block_timestamp());
        } else {
            let mut get_started_at_update = get_started_at.unwrap();
            self.get_started_at.remove(&new_round);
            self.get_started_at
                .insert(&new_round, env::block_timestamp());
        }
    }

    pub fn update_round_data(
        &mut self,
        _round_id: U128,
        _answer: U128,
        _timestamp: U128,
        _started_at: U128,
    ) {
        self.latest_answer = _answer;
        self.latest_timestamp = env::block_timestamp();
        self.latest_round = _round_id;

        let get_answer = self.get_answer.get(&self.latest_round);
        if get_answer.is_none() {
            env::panic(b"Answer doesn't exist");
        } else {
            let mut get_answer_update = get_answer.unwrap();
            self.get_answer.remove(&self.latest_round);
            self.get_answer.insert(&self.latest_round, _answer);
        }

        let get_timestamp = self.get_timestamp.get(&self.latest_round);
        if get_timestamp.is_none() {
            env::panic(b"timestget_timestamp doesn't exist");
        } else {
            let mut get_timestamp_update = get_timestamp.unwrap();
            self.get_timestamp.remove(&self.latest_round);
            self.get_timestamp
                .insert(&self.latest_round, _timestget_timestamp);
        }

        let get_started_at = self.get_started_at.get(&self.latest_round);
        if get_started_at.is_none() {
            env::panic(b"timestget_started_at doesn't exist");
        } else {
            let mut get_started_at_update = get_started_at.unwrap();
            self.get_started_at.remove(&self.latest_round);
            self.get_started_at
                .insert(&self.latest_round, _timestget_started_at);
        }
    }

    pub fn get_round_data(&self, _round_id: U64) -> (u64, u128, u128, u128, u128) {
        let round_id_u64: u64 = _round_id.into();
        let answer = self.get_answers.get(&round_id_u64);
        if answer.is_none() {
            env::panic(b"Answer not available");
        }
        let started_at = self.get_started_at.get(&round_id_u64);
        if timestamp.is_none() {
            env::panic(b"Timestamp not available");
        }
        let timestamp = self.get_timestamp.get(&round_id_u64);
        if timestamp.is_none() {
            env::panic(b"Timestamp not available");
        }

        return (
            round_id_u64,
            answer.unwrap(),
            started_at.unwrap(),
            timestamp.unwrap(),
            round_id_u64,
        );
    }

    pub fn latest_round_data(&self) -> (u64, u128, u128, u128, u128) {
        let round_id_u64: u64 = self.latest_round.into();
        let answer = self.get_answers.get(&self.latest_round);
        if answer.is_none() {
            env::panic(b"Answer not available");
        }
        let started_at = self.get_started_at.get(&self.latest_round);
        if timestamp.is_none() {
            env::panic(b"Timestamp not available");
        }
        let timestamp = self.get_timestamp.get(&self.latest_round);
        if timestamp.is_none() {
            env::panic(b"Timestamp not available");
        }

        return (
            round_id_u64,
            answer.unwrap(),
            started_at.unwrap(),
            timestamp.unwrap(),
            round_id_u64,
        );
    }

    pub fn get_description(&self) -> String {
        "v0.6/tests/MockV3Aggregator.sol"
    }
}
