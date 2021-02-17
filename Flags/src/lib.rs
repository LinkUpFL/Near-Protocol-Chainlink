use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Flags {
    pub raisingAccessController: AccountId,
    pub owner: AccountId,
    flags: LookupMap<AccountId, bool>,
    pub checkEnabled: bool,
    accessList: LookupMap<AccountId, bool>
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

        let mut result = Self {
            raisingAccessController: "".to_string(),
            owner: owner_id,
            flags: LookupMap::new(b"flags".to_vec()),
            checkEnabled: true,
            accessList: LookupMap::new(b"accessList".to_vec())
        };

        result.setRaisingAccessController(racAddress);
        result
    }

    pub fn getFlag(&self, subject: AccountId) -> bool {
        let flag = self.flags.get(&subject);
        if flag.is_none() {
            env::panic(b"The subject is invalid.");
        }
        flag.unwrap()
    }

    pub fn getFlags(&self, subjects: Vec<AccountId>) -> Vec::<bool> {
        let mut responses: Vec::<bool>;
        for i in 0..subjects.len() {
            let flag_option = self.flags.get(&subjects[i]);
            if flag_option.is_none() {
                env::panic(b"The subject is invalid.");
            }
            let flag = flag_option.unwrap();
            responses[i] = flag;
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
            let flag_option = self.flags.get(&subject);
            if flag_option.is_none() {
                env::panic(b"The subject is invalid.");
            }
            let flag = flag_option.unwrap();
            flag = false;
        }
    }

    pub fn setRaisingAccessController(&mut self, racAddress: AccountId) {
        self.onlyOwner();
        let previous: AccountId = self.raisingAccessController;

        if previous != racAddress {
            self.raisingAccessController = racAddress;
        }
    }

    // PRIVATE

    pub fn hasAccess(&self, _user: AccountId) -> bool {
        let oracle_id_option = self.accessList.get(&_user);
        if oracle_id_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        let oracle_id = oracle_id_option.unwrap();
        let userHasAccess = self.accessList.get(&_user);
            if userHasAccess.is_none() {
                env::panic(b"The subject is invalid.");
        }
        userHasAccess.unwrap() || !self.checkEnabled
    }

    fn allowedToRaiseFlags(&self) -> bool {
        env::predecessor_account_id() == self.owner || self.hasAccess(env::predecessor_account_id())
    }

    fn tryToRaiseFlag(&mut self, subject: AccountId) {
        let flag_option = self.flags.get(&subject);
        if flag_option.is_none() {
            let flag = flag_option.unwrap();
            flag = true;
        }
    }

    fn onlyOwner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }
}
