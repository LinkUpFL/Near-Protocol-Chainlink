use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::{self, json};
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::{Promise, PromiseOrValue, PromiseResult};
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13
const DEFAULT_GAS: u64 = 300_000_000_000_000;

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
            false
        } else {
            flag.unwrap()
        }
    }

    pub fn get_flags(&self, subjects: Vec<AccountId>) -> Vec<bool> {
        self.check_access();
        // let subjects_length: usize = subjects.len();
        // let mut responses: Vec<bool> = Vec::with_capacity(subjects_length);
        // let subjects_length: usize = subjects.len();
        let mut responses: Vec<bool> = Vec::new();
        for i in 0..subjects.len() {
            let flag = self.flags.get(&subjects[i]);
            if flag.is_none() {
                responses.push(false);
            } else {
                responses.push(flag.unwrap());
            }
        }
        return responses;
    }

    pub fn raise_flag(&mut self, subject: AccountId) {
        if env::predecessor_account_id() == self.owner {
            self.try_to_raise_flag(subject);
        } else {
            assert!(
                env::is_valid_account_id(self.raising_access_controller.as_bytes()),
                "Not allowed to raise flags"
            );
            let has_access_promise = env::promise_create(
                self.raising_access_controller.clone(),
                b"has_access",
                json!({ "_user": env::predecessor_account_id() })
                    .to_string()
                    .as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            let has_access_promise_results = env::promise_then(
                has_access_promise,
                env::current_account_id(),
                b"allowed_to_raise_flag_promise_result",
                json!({ "subject": subject }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            env::promise_return(has_access_promise_results);
        }
    }

    pub fn raise_flags(&mut self, subjects: Vec<AccountId>) {
        if env::predecessor_account_id() == self.owner {
            for i in 0..subjects.len() {
                self.try_to_raise_flag(subjects[i].clone());
            }
        } else {
            assert!(
                env::is_valid_account_id(self.raising_access_controller.as_bytes()),
                "Not allowed to raise flags"
            );
            let has_access_promise = env::promise_create(
                self.raising_access_controller.clone(),
                b"has_access",
                json!({ "_user": env::predecessor_account_id() })
                    .to_string()
                    .as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            let has_access_promise_results = env::promise_then(
                has_access_promise,
                env::current_account_id(),
                b"allowed_to_raise_flags_promise_result",
                json!({ "subjects": subjects }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            env::promise_return(has_access_promise_results);
        }
    }

    pub fn allowed_to_raise_flag_promise_result(&mut self, subject: AccountId) {
        // assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_allowed_to_raise_flags_promise_reult: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("Promise with index 0 failed"),
        };
        let allowed: bool =
            serde_json::from_slice(&get_allowed_to_raise_flags_promise_reult).unwrap();
        if !allowed {
            env::panic(b"Not allowed to raise flags");
        } else {
            self.try_to_raise_flag(subject);
        }
    }

    pub fn allowed_to_raise_flags_promise_result(&mut self, subjects: Vec<AccountId>) {
        // assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_allowed_to_raise_flags_promise_reult: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("Promise with index 0 failed"),
        };
        let allowed: bool =
            serde_json::from_slice(&get_allowed_to_raise_flags_promise_reult).unwrap();
        if !allowed {
            env::panic(b"Not allowed to raise flags");
        } else {
            for i in 0..subjects.len() {
                self.try_to_raise_flag(subjects[i].clone());
            }
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
                env::log(format!("{}", &subjects[i]).as_bytes());
            }
        }
    }
    pub fn get_raising_access_controller(&self) -> String {
        self.raising_access_controller.clone()
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
            env::panic(b"Did not find the user to remove.");
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

    // Commented out to get code working
    // fn allowed_to_raise_flags(&self) -> PromiseOrValue<bool> {
    // }

    fn try_to_raise_flag(&mut self, subject: AccountId) {
        let flag = self.flags.get(&subject);
        if flag.is_none() {
            self.flags.insert(&subject, &true);
            env::log(format!("{}", &subject).as_bytes());
        } else {
            if flag.unwrap() == false {
                self.flags.insert(&subject, &true);
                env::log(format!("{}", &subject).as_bytes());
            }
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
