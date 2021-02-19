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
    pub flags: LookupMap<AccountId, bool>
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

        let mut result = Self {
            owner: owner_id,
            flags: LookupMap::new(b"flags".to_vec()),
        };
        result.flags.insert(&"testing11.near".to_string(), &true);
        result.flags.insert(&"testing22.near".to_string(), &false);
        result
    }

    pub fn lower_flags(&mut self, subjects: Vec<AccountId>) {
        for i in 0..subjects.len() {            
            let subject = self.flags.get(&subjects[i]);
            if subject.is_none() {
                env::panic(b"The subject doesnt exist");
            }
            else if subject.unwrap() == true {
                self.flags.insert(&subjects[i], &false);
            }
            else {
                self.flags.insert(&subjects[i], &true);
            }
        }
    }

    pub fn get_flag(&self, account: AccountId) -> bool {
        let individual_flag = self.flags.get(&account);
        if individual_flag.is_none() {
            env::panic(b"The individual_flag doesnt exist")
        }
        else if individual_flag.unwrap() == true {
            true
        }
        else {
            false
        }
    }
}
 