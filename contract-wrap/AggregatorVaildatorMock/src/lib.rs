use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PromiseResult};
use std::convert::TryInto;
use std::str;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AggregatorValidatorMock {
    pub previous_round_id: u128,
    pub previous_answer: u128,
    pub current_round_id: u128,
    pub current_answer: i128,
}

impl Default for AggregatorValidatorMock {
    fn default() -> Self {
        panic!("AggregatorValidatorMock should be initialized before usage");
    }
}

#[near_bindgen]
impl AggregatorValidatorMock {

    fn validate(
        &self,
        previous_round_id: u128,
        previous_answer: u128,
        current_round_id: u128,
        current_answer: i128,
    ) -> bool {
        env::log(
            format!(
                "{}, {}, {}, {}",
                previous_round_id,
                previous_answer,
                current_round_id,
                current_answer
            )
            .as_bytes(),
        );
        true
    }
}

