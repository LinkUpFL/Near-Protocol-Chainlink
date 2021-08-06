use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::{self, json};
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::{Promise, PromiseOrValue, PromiseResult};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct flags_test_helper {
    pub flags: AccountId,
}

impl Default for flags_test_helper {
    fn default() -> Self {
        panic!("flags_test_helper should be initialized before usage")
    }
}

#[near_bindgen]
impl flags_test_helper {
    #[init]
    pub fn new(flags_contract: AccountId) -> Self {
        assert!(
            env::is_valid_account_id(flags_contract.as_bytes()),
            "Flag's account ID is invalid"
        );
        let result = Self {
            flags: flags_contract.to_string(),
        };
        result
    }

    pub fn get_flag(&self, subject: AccountId) {
        let get_flag_promise = env::promise_create(
            self.flags.clone(),
            b"get_flag",
            json!({ "subject": subject }).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let get_flag_promise_result = env::promise_then(
            get_flag_promise,
            env::current_account_id(),
            b"get_flag_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(get_flag_promise_result);
    }

    pub fn get_flag_results(&self) -> bool {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_flag_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let subject: bool = serde_json::from_slice(&get_flag_promise_result).unwrap();
        subject
    }


    pub fn get_flags(&self, subjects: Vec<AccountId>) {
        let get_flags_promise = env::promise_create(
            self.flags.clone(),
            b"get_flags",
            json!({ "subjects": subjects }).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let get_flags_promise_result = env::promise_then(
            get_flags_promise,
            env::current_account_id(),
            b"get_flags_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(get_flags_promise_result);
    }

    pub fn get_flags_results(&self) -> Vec<bool> {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_flags_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let subjects: Vec<bool> = serde_json::from_slice(&get_flags_promise_result).unwrap();
        subjects
    }

}
