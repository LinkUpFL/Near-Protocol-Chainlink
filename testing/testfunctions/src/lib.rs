use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TestingFunctions {
    pub owner: AccountId,
    flags: LookupMap<AccountId, bool>,
}

impl Default for TestingFunctions {
    fn default() -> Self {
        panic!("TestingFunctions should be initialized before usage")
    }
}

#[near_bindgen]
impl TestingFunctions {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let result = Self {
            owner: owner_id,
            flags: LookupMap::new(b"flags".to_vec()),
        };

        result
    }

    pub fn lower_flags(&mut self, subjects: Vec<AccountId>) {
        for i in 0..subjects.len() {            
            let subject: bool = self.flags.get(&subjects[i]).unwrap();
            if subject == false {
                env::panic(b"The flag is already false.");
            }
            let opposite_val: bool = self.flags.remove(&subjects[i]).unwrap();
            self.flags.insert(&subjects[i], &!opposite_val);
        }
    }

}
 