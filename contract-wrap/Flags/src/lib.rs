use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen, AccountId};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Flags {
    pub raising_access_controller: AccountId,
    pub owner: AccountId,
    pending_owner: AccountId,
    flags: LookupMap<AccountId, bool>,
    pub check_enabled: bool,
    access_list: LookupMap<AccountId, bool>,
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
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        assert!(
            rac_address == "" || env::is_valid_account_id(rac_address.as_bytes()),
            "rac_address account ID is invalid"
        );
        assert!(!env::state_exists(), "Already initialized");
        let mut result = Self {
            raising_access_controller: "".to_string(),
            owner: owner_id,
            pending_owner: "".to_string(),
            flags: LookupMap::new(b"flags".to_vec()),
            check_enabled: true,
            access_list: LookupMap::new(b"access_list".to_vec()),
        };
        result.set_raising_access_controller(rac_address);
        result
    }

    pub fn get_flag(&self, subject: AccountId) -> bool {
        self.check_access();
        let flag = self.flags.get(&subject);
        if flag.is_none() {
            env::panic(b"The subject is invalid.");
        }
        flag.unwrap()
    }

    pub fn get_flags(&self, subjects: Vec<AccountId>) -> Vec<bool> {
        self.check_access();
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
        if env::predecessor_account_id() != env::current_account_id() {
            self.only_owner();
        }
        let previous: AccountId = String::from(&self.raising_access_controller) as AccountId;
        let init_rac_address: AccountId = rac_address.clone();
        if previous != rac_address {
            self.raising_access_controller = rac_address;
            env::log(format!("{}, {}", previous, init_rac_address).as_bytes());
        }
    }

    pub fn has_access(&self, _user: AccountId) -> bool {
        if !self.check_enabled {
            !self.check_enabled
        } else {
            let user_option = self.access_list.get(&_user);
            if user_option.is_none() {
                return false;
            }
            let user = user_option.unwrap();
            user
        }
    }

    pub fn add_access(&mut self, _user: AccountId) {
        self.only_owner();

        let user_option = self.access_list.get(&_user);
        if user_option.is_none() {
            self.access_list.insert(&_user, &true);
        }
    }

    pub fn remove_access(&mut self, _user: AccountId) {
        self.only_owner();

        let user_option = self.access_list.get(&_user);
        if user_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        self.access_list.insert(&_user, &false);
    }

    pub fn enable_access_check(&mut self) {
        self.only_owner();

        if !self.check_enabled {
            self.check_enabled = true;
        }
    }

    pub fn disable_access_check(&mut self) {
        self.only_owner();

        if self.check_enabled {
            self.check_enabled = false;
        }
    }

    // PRIVATE

    fn allowed_to_raise_flags(&self) -> bool {
        env::predecessor_account_id() == self.owner
            || self.has_access(env::predecessor_account_id())
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

    fn only_owner(&self) {
        assert_eq!(
            self.owner,
            env::predecessor_account_id(),
            "Only callable by owner."
        );
    }

    fn check_access(&self) {
        assert!(self.has_access(env::predecessor_account_id()), "No access")
    }

    pub fn transfer_ownership(&mut self, _to: AccountId) {
        self.only_owner();
        let init_to: AccountId = _to.clone();
        self.pending_owner = _to;
        env::log(format!("{}, {}", self.owner, init_to).as_bytes());
    }

    pub fn accept_ownership(&mut self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.pending_owner,
            "Must be proposed owner"
        );
        let old_owner: AccountId = self.owner.clone();
        self.owner = env::predecessor_account_id();
        self.pending_owner = "".to_string();
        env::log(format!("{}, {}", old_owner, env::predecessor_account_id()).as_bytes());
    }
}
