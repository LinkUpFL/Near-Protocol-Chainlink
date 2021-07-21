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
pub struct FluxAggregatorTestHelper {
    pub requested_round_id: u64,
}

impl Default for FluxAggregatorTestHelper {
    fn default() -> Self {
        panic!("FluxAggregatorTestHelper should be initialized before usage")
    }
}

#[near_bindgen]
impl FluxAggregatorTestHelper {
    #[init]
    pub fn new() -> Self {
        let result = Self {
            requested_round_id: 0,
        };
        result
    }

    pub fn read_oracle_round_state(&self, _aggregator: AccountId, _oracle: AccountId) {
        let read_oracle_round_state_promise = env::promise_create(
            _aggregator.to_string(),
            b"oracle_round_state",
            json!({ "_oracle": _oracle.to_string(), "_queried_round_id": 0.to_string()}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_oracle_round_state_promise_result = env::promise_then(
            read_oracle_round_state_promise,
            env::current_account_id(),
            b"read_oracle_round_state_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_oracle_round_state_promise_result);
    }

    pub fn read_oracle_round_state_results(&self) -> (bool, u64, u128, u64, u64, u128, u64, u128) {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_oracle_round_state_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: (bool, u64, u128, u64, u64, u128, u64, u128) = serde_json::from_slice(&read_oracle_round_state_promise_result).unwrap();
        results
    }

    pub fn read_get_round_data(&self, _aggregator: AccountId, _round_id: u64) {
        let read_get_round_data_promise = env::promise_create(
            _aggregator.to_string(),
            b"get_round_data",
            json!({ "_round_id": _round_id.to_string()}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_get_round_data_promise_result = env::promise_then(
            read_get_round_data_promise,
            env::current_account_id(),
            b"read_get_round_data_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_get_round_data_promise_result);
    }

    pub fn read_get_round_data_results(&self) -> (u64, u128, u64, u64, u64) {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_get_round_data_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: (u64, u128, u64, u64, u64) = serde_json::from_slice(&read_get_round_data_promise_result).unwrap();
        results
    }


    pub fn read_latest_round_data(&self, _aggregator: AccountId) {
        let read_latest_round_data_promise = env::promise_create(
            _aggregator.to_string(),
            b"latest_round_data",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_latest_round_data_promise_result = env::promise_then(
            read_latest_round_data_promise,
            env::current_account_id(),
            b"read_latest_round_data_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_latest_round_data_promise_result);
    }

    pub fn read_latest_round_data_results(&self) -> (u64, u128, u64, u64, u64) {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_latest_round_data_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: (u64, u128, u64, u64, u64) = serde_json::from_slice(&read_latest_round_data_promise_result).unwrap();
        results
    }


    pub fn read_latest_answer(&self, _aggregator: AccountId) {
        let read_latest_answer_promise = env::promise_create(
            _aggregator.to_string(),
            b"latest_answer",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_latest_answer_promise_result = env::promise_then(
            read_latest_answer_promise,
            env::current_account_id(),
            b"read_latest_answer_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_latest_answer_promise_result);
    }

    pub fn read_latest_answer_results(&self) -> u128 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_latest_answer_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: u128 = serde_json::from_slice(&read_latest_answer_promise_result).unwrap();
        results
    }

    pub fn read_latest_timestamp(&self, _aggregator: AccountId) {
        let read_latest_timestamp_promise = env::promise_create(
            _aggregator.to_string(),
            b"latest_timestamp",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_latest_timestamp_promise_result = env::promise_then(
            read_latest_timestamp_promise,
            env::current_account_id(),
            b"read_latest_timestamp_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_latest_timestamp_promise_result);
    }

    pub fn read_latest_timestamp_results(&self) -> u64 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_latest_timestamp_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: u64 = serde_json::from_slice(&read_latest_timestamp_promise_result).unwrap();
        results
    }

    pub fn read_latest_round(&self, _aggregator: AccountId) {
        let read_latest_round_promise = env::promise_create(
            _aggregator.to_string(),
            b"latest_round",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_latest_round_promise_result = env::promise_then(
            read_latest_round_promise,
            env::current_account_id(),
            b"read_latest_round_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_latest_round_promise_result);
    }

    pub fn read_latest_round_results(&self) -> u64 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_latest_round_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: u64 = serde_json::from_slice(&read_latest_round_promise_result).unwrap();
        results
    }

    pub fn request_new_round(&self, _aggregator: AccountId) {
        let request_new_round_promise = env::promise_create(
            _aggregator.to_string(),
            b"request_new_round",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let request_new_round_promise_result = env::promise_then(
            request_new_round_promise,
            env::current_account_id(),
            b"request_new_round_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(request_new_round_promise_result);
    }

    pub fn request_new_round_results(&mut self) -> u64 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let request_new_round_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: u64 = serde_json::from_slice(&request_new_round_promise_result).unwrap();
        self.requested_round_id = results;
        results
    }

    pub fn read_get_answer(&self, _aggregator: AccountId, _round_id: u64) {
        let read_get_answer_promise = env::promise_create(
            _aggregator.to_string(),
            b"get_answer",
            json!({"_round_id": _round_id.to_string()}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_get_answer_promise_result = env::promise_then(
            read_get_answer_promise,
            env::current_account_id(),
            b"read_get_answer_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_get_answer_promise_result);
    }

    pub fn read_get_answer_results(&self) -> u64 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_get_answer_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: u64 = serde_json::from_slice(&read_get_answer_promise_result).unwrap();
        results
    }

    pub fn read_get_timestamp(&self, _aggregator: AccountId, _round_id: u64) {
        let read_get_timestamp_promise = env::promise_create(
            _aggregator.to_string(),
            b"get_timestamp",
            json!({"_round_id": _round_id.to_string()}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let read_get_timestamp_promise_result = env::promise_then(
            read_get_timestamp_promise,
            env::current_account_id(),
            b"read_get_timestamp_results",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        env::promise_return(read_get_timestamp_promise_result);
    }

    pub fn read_get_timestamp_results(&self) -> u64 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let read_get_timestamp_promise_result: Vec<u8> = match env::promise_result(0) {
            PromiseResult::Successful(_x) => _x,
            _x => panic!("No access"),
        };
        let results: u64 = serde_json::from_slice(&read_get_timestamp_promise_result).unwrap();
        results
    }

}
