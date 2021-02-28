use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use hex::{encode};
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
        let request_id = hex::encode(to_encode);

        let commitment_option = self.commitments.get(&request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment == "", "Must use a unique ID");
        let expiration: u128 = env::block_timestamp() + EXPIRY_TIME.into();

        let commitment_to_encode: Base64String = [payment_u128.to_string(), _callbackAddress, _callbackFunctionId, expiration.to_string()].join("\n");
        let to_insert: Base64String = hex::encode(commitment_to_encode);
        self.commitments.insert(&request_id, &to_insert);
    }

    pub fn fulfillOracleRequest(&mut self, _request_id: Base64String, _payment: U128, _callbackAddress: AccountId, _callbackFunctionId: Base64String, _expiration: U128, _data: Base64String) -> bool {
        self.isValidRequest(_request_id);
        let payment_u128: u128 = _payment.into();
        let expiration_u128: u128 = _expiration.into();
        let paramsHash: Base64String = hex::encode(env::keccak256(payment_u128, _callbackAddress, _callbackFunctionId, expiration_u128));
        let commitment_option = self.commitments.get(&_request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment == paramsHash, "Params do not match request ID");
        self.withdrawableTokens = self.withdrawableTokens + payment_u128;
        self.commitments.remove(&_request_id);
        // gasleft

        //call back
    }

    pub fn getAuthorizationStatus(&self, _node: AccountId) -> bool {
        let node_option = self.authorizedNodes.get(&_node);
        if node_option.is_none() {
            env::panic(b"Could not find node.");
        }
        node_option.unwrap()
    }

    pub fn setFulfillmentPermission(&mut self, _node: AccountId, _allowed: bool) {
        self.onlyOwner();

        let node_option = self.authorizedNodes.get(&_node);
        if node_option.is_none() {
            env::panic(b"Could not find node.");
        }
        self.authorizedNodes.insert(&_node, &_allowed);
    }

    pub fn withdraw(&mut self, _recipient: AccountId, _amount: U128) {
        self.onlyOwner();
        self.hasAvailableFunds(_amount);
        let amount_u128: u128 = _amount.into();

        self.withdrawableTokens = self.withdrawableTokens - amount_u128;
        //assert!()
    }

    pub fn withdrawable(&mut self) -> u128 {
        self.withdrawableTokens
    }

    pub fn cancelOracleRequest(&mut self, _request_id: Base64String, _payment: U128, _callbackFunc: Base64String, _expiration: U128) {
        let payment_u128: u128 = _payment.into();
        let expiration_u128: u128 = _expiration.into();
        let paramsHash: Base64String = hex::encode(env::keccak256(payment_u128, _callbackAddress, _callbackFunc, expiration_u128));
        let commitment_option = self.commitments.get(&_request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(paramsHash == commitment, "Params do not match request ID");
        assert!(expiration_u128 <= env::block_timestamp().into());

        self.commitments.remove(&_request_id);

        //assert linkToken
    }

    pub fn getChainlinkToken(&self) -> AccountId {
        self.linkToken
    }

    // MODIFIERS

    fn hasAvailableFunds(&self, _amount: U128) {
        let amount_u128: u128 = _amount.into();
        assert!(self.withdrawableTokens >= amount_u128);
    }

    fn isValidRequest(&self, _request_id: Base64String) {
        let commitment_option = self.commitments.get(&_request_id);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment != "", "Must have a valid request_id");
    }

    fn onlyAuthorizedNode(&self) {
        let node_option = self.authorizedNodes.get(&env::predecessor_account_id());
        assert!(node_option.is_some() || env::predecessor_account_id() == self.owner, "Not an authorized node to fulfill requests");
    }

    fn checkCallbackAddress(&self, _to: &AccountId) {
        assert_ne!(_to, &self.linkToken, "Cannot callback to LINK.");
        assert_ne!(_to, &env::current_account_id(), "Callback address cannot be the oracle contract.");
    }

    fn onlyOwner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }

    // Test to see if this works
    fn onlyLink(&self) {
        assert_eq!(env::predecessor_account_id(), self.linkToken, "Must use LINK token.");
    }
}
