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

    pub fn raiseFlag(&mut self, subject: AccountId) {
        assert!(self.allowedToRaiseFlags(), "Not allowed to raise flags");

        self.tryToRaiseFlag(subject);
    }

    pub fn raiseFlags(&mut self, subjects: AccountId[]) {
        assert!(self.allowedToRaiseFlags(), "Not allowed to raise flags");

        for i in 0..subjects.len() {
            self.tryToRaiseFlag(subjects[i]);
        }
    }

    pub fn lowerFlags(&mut self, subjects: AccountId[]) {
        self.onlyOwner();
        for i in 0..subjects.len() {
            let subject: AccountId = subjects[i];

            if(self.flags[subject]) {
                self.flags[subject] = false;
            }
        }
    }

    pub fn setRaisingAccessController(&mut self, racAddress: AccountId) {
        self.onlyOwner();
        let previous: AccountId = self.raisingAccessController;

        if(previous != racAddress) {
            self.raisingAccessController = racAddress;
        }
    }

    // PRIVATE

    fn allowedToRaiseFlags(&mut self) -> bool {
        // Check owner syntax and msg.data
        return env::predecessor_account_id() == owner || self.raisingAccessController.hasAccess(env::predecessor_account_id(), msg.data);
    }

    fn tryToRaiseFlag(&mut self, subject: AccountId) {
        if(!self.flags[subject]) {
            self.flags[subject] = true;
        }
    }

    fn onlyOwner(&mut self) {
        assert_eq!(env::signer_account_id(), env::current_account_id(), "Only contract owner can call this method.");
    }
}
