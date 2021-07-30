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
    pub current_answer: u128,
}

impl Default for AggregatorValidatorMock {
    fn default() -> Self {
        panic!("AggregatorValidatorMock should be initialized before usage");
    }
}

#[near_bindgen]
impl AggregatorValidatorMock {

    #[init]
    pub fn new() -> Self {
        let result = Self {
            previous_round_id: 0,
            previous_answer: 0,
            current_round_id: 0,
            current_answer: 0,
        };
        result
    }

    pub fn validate(
        &self,
        previous_round_id: U128,
        previous_answer: U128,
        current_round_id: U128,
        current_answer: U128,
    ) -> bool {
        env::log(
            format!(
                "{}, {}, {}, {}",
                u128::from(previous_round_id),
                u128::from(previous_answer),
                u128::from(current_round_id),
                u128::from(current_answer)
            )
            .as_bytes(),
        );
        true
    }
}

