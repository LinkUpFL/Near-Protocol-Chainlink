use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Flags {
    pub raising_access_controller: AccountId,
    pub owner: AccountId,
    flags: LookupMap<AccountId, bool>,
    pub check_enabled: bool,
    access_list: LookupMap<AccountId, bool>
}

impl Default for Flags {
    fn default() -> Self {
        panic!("Flags should be initialized before usage")
    }
}

#[near_bindgen]
impl Flags {
    #[init]
    pub fn new(owner_id: AccountId, rac_address: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(rac_address.as_bytes()), "recAddress account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");
        let result = Self {
            raising_access_controller: rac_address,
            owner: owner_id,
            flags: LookupMap::new(b"flags".to_vec()),
            check_enabled: true,
            access_list: LookupMap::new(b"access_list".to_vec())
        };
        result
    }

    pub fn get_flag(&self, subject: AccountId) -> bool {
        let flag = self.flags.get(&subject);
        if flag.is_none() {
            env::panic(b"The subject is invalid.");
        }
        flag.unwrap()
    }

    pub fn get_flags(&self, subjects: Vec<AccountId>) -> Vec::<bool> {
        let subjects_length: usize = subjects.len();
        let mut responses: Vec<bool> = Vec::with_capacity(subjects_length);
        for i in 0..subjects.len() {
            let flag = self.flags.get(&subjects[i]);
            if flag.is_none() {
                env::panic(b"The subject is invalid.");
            }
            responses[i] = flag.unwrap();
        }
        return responses;
    }

    pub fn raise_flag(&mut self, subject: AccountId) {
        assert!(self.allowed_to_raise_flags(), "Not allowed to raise flags");
        self.try_to_raise_flag(subject);
    }

    pub fn raise_flags(&mut self, subjects: Vec<AccountId>) {
        assert!(self.allowed_to_raise_flags(), "Not allowed to raise flags");

        for i in 0..subjects.len() {
            self.try_to_raise_flag(subjects[i].clone());
        }
    }

    pub fn lower_flags(&mut self, subjects: Vec<AccountId>) {
        self.only_owner();
        for i in 0..subjects.len() {            
            let subject = self.flags.get(&subjects[i]);
            if subject.is_none() {
                env::panic(b"The subject doesnt exist");
            }
            if subject.unwrap() == true {
                self.flags.insert(&subjects[i], &false);
            }
        }
    }
    pub fn set_raising_access_controller(&mut self, rac_address: AccountId) {
        self.only_owner();
        if self.raising_access_controller.clone() != rac_address {
            self.raising_access_controller = rac_address;
        }
    }

    // PRIVATE

    pub fn has_access(&self, user: AccountId) -> bool {
        let oracle_id_option = self.access_list.get(&user);
        if oracle_id_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        let user_has_access = self.access_list.get(&user);
            if user_has_access.is_none() {
                env::panic(b"The subject is invalid.");
        }
        user_has_access.unwrap() || !self.check_enabled
    }

    fn allowed_to_raise_flags(&self) -> bool {
        env::predecessor_account_id() == self.owner || self.has_access(env::predecessor_account_id())
    }

    fn try_to_raise_flag(&mut self, subject: AccountId) {
        let flag = self.flags.get(&subject);
        if flag.is_none() {
            env::panic(b"The subject is invalid.");
        }
        if flag.unwrap() == false {
            self.flags.insert(&subject, &true);
        }
    }

    fn only_owner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }
}
 
