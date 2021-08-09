use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub type Base64String = String;

const VERSION: u128 = 0;

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
        let transformed_answer: u128 = _answer.into();
        self.latest_answer = transformed_answer;
        self.latest_timestamp = u128::from(current_timestamp);
        self.latest_round = new_round;

        let get_answer = self.get_answer.get(&new_round);
        if get_answer.is_none() {
            self.get_answer.insert(&new_round, &transformed_answer);
        } else {
            self.get_answer.remove(&new_round);
            self.get_answer.insert(&new_round, &transformed_answer);
        }

        let get_timestamp = self.get_timestamp.get(&new_round);
        if get_timestamp.is_none() {
            self.get_timestamp
                .insert(&new_round, &u128::from(current_timestamp));
        } else {
            self.get_timestamp.remove(&new_round);
            self.get_timestamp
                .insert(&new_round, &u128::from(current_timestamp));
        }

        let get_started_at = self.get_started_at.get(&new_round);
        if get_started_at.is_none() {
            self.get_started_at
                .insert(&new_round, &u128::from(current_timestamp));
        } else {
            self.get_started_at.remove(&new_round);
            self.get_started_at
                .insert(&new_round, &u128::from(current_timestamp));
        }
    }

    pub fn update_round_data(
        &mut self,
        _round_id: U128,
        _answer: U128,
        _timestamp: U128,
        _started_at: U128,
    ) {
        let transformed_round: u128 = _round_id.into();
        let transformed_timestamp: u128 = u128::from(env::block_timestamp());
        let transformed_answer: u128 = _answer.into();
        let transformed_started_at: u128 = _started_at.into();

        self.latest_answer = transformed_answer;
        self.latest_timestamp = transformed_timestamp;
        self.latest_round = transformed_round;

        let get_answer = self.get_answer.get(&transformed_round);
        if get_answer.is_none() {
            self.get_answer.insert(&transformed_round, &transformed_answer);
        } else {
            self.get_answer.remove(&transformed_round);
            self.get_answer
                .insert(&transformed_round, &transformed_answer);
        }

        let get_timestamp = self.get_timestamp.get(&transformed_round);
        if get_timestamp.is_none() {
            self.get_timestamp.insert(&transformed_round, &transformed_timestamp);
        } else {
            self.get_timestamp.remove(&transformed_round);
            self.get_timestamp
                .insert(&transformed_round, &transformed_timestamp);
        }

        let get_started_at = self.get_started_at.get(&transformed_round);
        if get_started_at.is_none() {
            self.get_started_at.insert(&transformed_round, &transformed_started_at);
        } else {
            self.get_started_at.remove(&transformed_round);
            self.get_started_at
                .insert(&transformed_round, &transformed_started_at);
        }
    }

    pub fn get_round_data(&self, _round_id: U128) -> (u128, u128, u128, u128, u128) {
        let round_id: u128 = u128::from(_round_id);

        let answer = self.get_answer.get(&round_id);
        if answer.is_none() {
            env::panic(b"Answer not available");
        }

        let started_at = self.get_started_at.get(&round_id);
        if started_at.is_none() {
            env::panic(b"Timestamp not available");
        }

        let timestamp = self.get_timestamp.get(&round_id);
        if timestamp.is_none() {
            env::panic(b"Timestamp not available");
        }

        return (
            round_id,
            answer.unwrap(),
            started_at.unwrap(),
            timestamp.unwrap(),
            round_id
        );
    }

    pub fn latest_round_data(&self) -> (u128, u128, u128, u128, u128) {

        let answer = self.get_answer.get(&self.latest_round);
        if answer.is_none() {
            env::panic(b"Answer not available");
        }

        let started_at = self.get_started_at.get(&self.latest_round);
        if started_at.is_none() {
            env::panic(b"Timestamp not available");
        }

        let timestamp = self.get_timestamp.get(&self.latest_round);
        if timestamp.is_none() {
            env::panic(b"Timestamp not available");
        }

        return (
            self.latest_round,
            answer.unwrap(),
            started_at.unwrap(),
            timestamp.unwrap(),
            self.latest_round,
        );
    }

    pub fn latest_answer(&self) -> u128 {
        self.latest_answer
    }

    pub fn latest_timestamp(&self) -> u128 {
        self.latest_timestamp
    }

    pub fn latest_round(&self) -> u128 {
        self.latest_round
    }

    pub fn get_answer(&self,  _round_id: U128) -> u128 {
        let round_id: u128 = u128::from(_round_id);

        let answer = self.get_answer.get(&round_id);
        if answer.is_none() {
            env::panic(b"Answer not available");
        }
        answer.unwrap()
    }

    pub fn get_timestamp(&self,  _round_id: U128) -> u128 {
        let round_id: u128 = u128::from(_round_id);

        let timestamp = self.get_timestamp.get(&round_id);
        if timestamp.is_none() {
            env::panic(b"timestamp not available");
        }
        timestamp.unwrap()
    }

    pub fn get_description(&self) -> String {
        String::from("v0.6/tests/MockV3Aggregator.sol")
    }

    pub fn get_decimals(&self) -> u128 {
        self.decimals
    }
    pub fn get_version(&self) -> u128 {
        VERSION
    }


}
