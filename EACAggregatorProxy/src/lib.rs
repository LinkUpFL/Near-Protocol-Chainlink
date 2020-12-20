use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct EACAggregatorProxy {
    accessController: AccountId
}

#[near_bindgen]
impl EACAggregatorProxy {
    pub fn setController(&mut self, _accessController: AccountId) {
        self.accessController = _accessController;
    }

    pub fn latestAnswer(&mut self, message: String) -> i256 {
        // super.latestAnswer
        return
    }
}
