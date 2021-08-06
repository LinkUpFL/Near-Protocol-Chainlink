use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen, AccountId};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct simple_read_access_controller {
    pub owner: AccountId,
    pending_owner: AccountId,
    pub check_enabled: bool,
    access_list: LookupMap<AccountId, bool>,
}

impl Default for simple_read_access_controller {
    fn default() -> Self {
        panic!("simple_read_access_controller should be initialized before usage")
    }
}

#[near_bindgen]
impl simple_read_access_controller {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        assert!(!env::state_exists(), "Already initialized");
        let result = Self {
            owner: owner_id,
            pending_owner: "".to_string(),
            check_enabled: true,
            access_list: LookupMap::new(b"access_list".to_vec()),
        };
        result
    }

    pub fn has_access(&self, _user: AccountId) -> bool {
        if env::signer_account_id() != env::predecessor_account_id() {
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
        } else {
            true
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
