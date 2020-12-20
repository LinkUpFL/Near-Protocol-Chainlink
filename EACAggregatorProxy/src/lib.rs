use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Round {
    answer: i256,
    startedAt: u64,
    updatedAt: u64,
    answeredInRound: u32
}

#[derive(Serialize, Deserialize)]
pub struct RoundDetails {
    submissions: i256[],
    maxSubmissions: u32,
    minSubmissions: u32,
    timeout: u32,
    paymentAmount: u128
}

#[derive(Serialize, Deserialize)]
pub struct OracleStatus {
    withdrawable: u128,
    startingRound: u32,
    endingRound: u32,
    lastReportedRound: u32,
    lastStartedRound: u32,
    latestSubmission: i256,
    index: u16,
    admin: AccountId,
    pendingAdmin: AccountId
}

#[derive(Serialize, Deserialize)]
pub struct Requester {
    authorized: bool,
    delay: u32,
    lastStartedRound: u32
}

#[derive(Serialize, Deserialize)]
pub struct Funds {
    available: u128,
    allocated: u128
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct EACAggregatorProxy {
    paymentAmount: u128,
    maxSubmissionCount: u32,
    minSubmissionCount: u32,
    restartDelay: u32,
    timeout: u32,
    decimals: u8,
    description: Base64String,
    minSubmissionValue: i256,
    maxSubmissionValue: i256,
    version: u256
}

#[near_bindgen]
impl EACAggregatorProxy {
    pub fn latestAnswer(&mut self, message: String) -> i256 {
        let account_id = env::signer_account_id();
        self.records.insert(account_id, message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        self.records.get(&account_id).cloned()
    }
}
