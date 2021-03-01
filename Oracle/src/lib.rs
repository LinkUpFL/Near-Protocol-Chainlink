use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use std::str;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const EXPIRY_TIME: u64 = 5 * 60 * 1_000_000_000;

// max gas: 300_000_000_000_000

const MINIMUM_CONSUMER_GAS_LIMIT: u64 = 1_000_000_000;
const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13
const TRANSFER_FROM_NEAR_COST: u128 = 36_500_000_000_000_000_000_000; // 365 x 10^20

pub type Base64String = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Oracle {
    pub owner: AccountId,
    pub link_token: AccountId,
    commitments: LookupMap<Base64String, Base64String>,
    authorized_nodes: LookupMap<AccountId, bool>,
    withdrawable_tokens: u128
}

impl Default for Oracle {
    fn default() -> Self {
        panic!("Oracle should be initialized before usage")
    }
}

#[near_bindgen]
impl Oracle {
    #[init]
    pub fn new(link_id: AccountId, owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(link_id.as_bytes()), "Link token account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner: owner_id,
            link_token: link_id,
            commitments: LookupMap::new(b"commitments".to_vec()),
            authorized_nodes: LookupMap::new(b"authorized_nodes".to_vec()),
            withdrawable_tokens: 0_u128
        }
    }

    pub fn oracle_request(&mut self, _sender: AccountId, _payment: U128, _spec_id: Base64String, _callback_address: AccountId, _callback_function_id: Base64String, _nonce: U128, _data_version: U128, _data: Base64String) {
        self.only_link();
        self.check_callback_address(&_callback_address);

        let payment_u128: u128 = _payment.into();
        let nonce_u128: u128 = _nonce.into();
        let data_version_u128: u128 = _data_version.into();

        let request_id_to_encode: Base64String = [_sender, nonce_u128.to_string()].join("\n");
        let request_id: Base64String = hex::encode(request_id_to_encode);

        let commitment_option = self.commitments.get(&request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment == "", "Must use a unique ID");
        let expiration: u128 = (env::block_timestamp() + EXPIRY_TIME).into();

        let commitment_to_encode: Base64String = [payment_u128.to_string(), _callback_address, _callback_function_id, expiration.to_string()].join("\n");
        let to_insert: Base64String = hex::encode(commitment_to_encode);
        self.commitments.insert(&request_id, &to_insert);
    }

    pub fn fulfill_oracle_request(&mut self, _request_id: Base64String, _payment: U128, _callback_address: AccountId, _callback_function_id: Base64String, _expiration: U128, _data: Base64String) -> bool {
        self.is_valid_request(_request_id);
        let payment_u128: u128 = _payment.into();
        let expiration_u128: u128 = _expiration.into();
        let params_hash_to_encode: Base64String = [payment_u128.to_string(), _callback_address, _callback_function_id, expiration_u128.to_string()].join("\n");
        let params_hash: Base64String = hex::encode(params_hash_to_encode);
        let commitment_option = self.commitments.get(&_request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment == params_hash, "Params do not match request ID");
        self.withdrawable_tokens = self.withdrawable_tokens + payment_u128;
        self.commitments.remove(&_request_id);
        // gasleft

        //call back
    }

    pub fn get_authorization_status(&self, _node: AccountId) -> bool {
        let node_option = self.authorized_nodes.get(&_node);
        if node_option.is_none() {
            env::panic(b"Could not find node.");
        }
        node_option.unwrap()
    }

    pub fn set_fulfillment_permission(&mut self, _node: AccountId, _allowed: bool) {
        self.only_owner();

        let node_option = self.authorized_nodes.get(&_node);
        if node_option.is_none() {
            env::panic(b"Could not find node.");
        }
        self.authorized_nodes.insert(&_node, &_allowed);
    }

    pub fn withdraw(&mut self, _recipient: AccountId, _amount: U128) {
        self.only_owner();
        self.has_available_funds(_amount);
        let amount_u128: u128 = _amount.into();

        self.withdrawable_tokens = self.withdrawable_tokens - amount_u128;
        //assert!()
    }

    pub fn withdrawable(&mut self) -> u128 {
        self.withdrawable_tokens
    }

    pub fn cancel_oracle_request(&mut self, _request_id: Base64String, _payment: U128, _callback_func: Base64String, _expiration: U128) {
        let payment_u128: u128 = _payment.into();
        let expiration_u128: u128 = _expiration.into();
        let params_hash_to_encode: Base64String = [payment_u128.to_string(), env::predecessor_account_id(), _callback_func, expiration_u128.to_string()].join("\n");
        let params_hash: Base64String = hex::encode(params_hash_to_encode);
        let commitment_option = self.commitments.get(&_request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(params_hash == commitment, "Params do not match request ID");
        assert!(expiration_u128 <= env::block_timestamp().into());

        self.commitments.remove(&_request_id);

        //assert link_token
    }

    pub fn getChainlink_token(&self) -> AccountId {
        self.link_token
    }

    // MODIFIERS

    fn has_available_funds(&self, _amount: U128) {
        let amount_u128: u128 = _amount.into();
        assert!(self.withdrawable_tokens >= amount_u128);
    }

    fn is_valid_request(&self, _request_id: Base64String) {
        let commitment_option = self.commitments.get(&_request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment != "", "Must have a valid request_id");
    }

    fn onlyauthorized_nodes(&self) {
        let node_option = self.authorized_nodes.get(&env::predecessor_account_id());
        assert!(node_option.is_some() || env::predecessor_account_id() == self.owner, "Not an authorized node to fulfill requests");
    }

    fn check_callback_address(&self, _to: &AccountId) {
        assert_ne!(_to, &self.link_token, "Cannot callback to LINK.");
        assert_ne!(_to, &env::current_account_id(), "Callback address cannot be the oracle contract.");
    }

    fn only_owner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }

    // Test to see if this works
    fn only_link(&self) {
        assert_eq!(env::predecessor_account_id(), self.link_token, "Must use LINK token.");
    }
}
