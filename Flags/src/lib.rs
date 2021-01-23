use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Flags {
    pub raisingAccessController: AccountId,
    flags: LookupMap<AccountId, bool>
}

#[near_bindgen]
impl Flags {
    pub fn getFlag(&self, subject: AccountId) -> bool {
        self.flags[subject]
    }

    pub fn getFlags(&self, subjects: AccountId[]) -> bool {
        let responses: bool[subjects.len()];
        for i in 0..subjects.len() {
            responses[i] = self.flags[subjects[i]];
        }
        return responses;
    }
}
