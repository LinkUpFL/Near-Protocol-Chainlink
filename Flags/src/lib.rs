use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{AccountId, env, near_bindgen};
use std::str;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Flags {
    pub raisingAccessController: AccountId,
    pub owner: AccountId,
    flags: LookupMap<AccountId, bool>
}

impl Default for Flags {
    fn default() -> Self {
        panic!("Flags should be initialized before usage")
    }
}

#[near_bindgen]
impl Flags {
    #[init]
    pub fn new(owner_id: AccountId, racAddress: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(racAddress.as_bytes()), "recAddress account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        self.setRaisingAccessController(&racAddress);

        Self {
            owner: owner_id
        }
    }

    pub fn getFlag(&self, subject: AccountId) -> bool {
        self.flags[subject]
    }

    pub fn getFlags(&self, subjects: Vec<AccountId>) -> bool {
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

    pub fn raiseFlags(&mut self, subjects: Vec<AccountId>) {
        assert!(self.allowedToRaiseFlags(), "Not allowed to raise flags");

        for i in 0..subjects.len() {
            self.tryToRaiseFlag(subjects[i]);
        }
    }

    pub fn lowerFlags(&mut self, subjects: Vec<AccountId>) {
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
        env::predecessor_account_id() == owner || self.raisingAccessController.hasAccess(env::predecessor_account_id());
    }

    fn tryToRaiseFlag(&mut self, subject: AccountId) {
        if(!self.flags[subject]) {
            self.flags[subject] = true;
        }
    }

    fn onlyOwner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }
}
