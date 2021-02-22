use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{AccountId, env, near_bindgen, Promise};
use near_sdk::wee_alloc::{WeeAlloc};
use near_sdk::serde_json::json;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CrossContractTesting {
    pub owner: AccountId,
}

impl Default for CrossContractTesting {
    fn default() -> Self {
        panic!("CrossContractTesting should be initialized before usage")
    }
}

#[near_bindgen]
impl CrossContractTesting {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let result = Self {
            owner: owner_id,
        };
        result
    }

    pub fn call_testfunctions(&self) -> Promise {
        Promise::new("testfunctions10.nolanjacobson.testnet".to_string())
        .function_call(
            b"get_flag".to_vec(),
            json!({"account": "testing11.near"}).to_string().as_bytes().to_vec(),
            0,
            SINGLE_CALL_GAS,
        )
        .as_return()
    }

}
 