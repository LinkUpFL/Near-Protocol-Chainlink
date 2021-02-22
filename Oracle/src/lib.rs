use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use near_sdk::base64::{decode};
use std::str;
use num_traits::pow;

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
    pub linkToken: AccountId,
    commitments: LookupMap<Base64String, Base64String>,
    authorizedNodes: LookupMap<AccountId, bool>,
    withdrawableTokens: u128
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
            linkToken: link_id,
            commitments: LookupMap::new(b"commitments".to_vec()),
            authorizedNodes: LookupMap::new(b"authorizedNodes".to_vec()),
            withdrawableTokens: 0_u128
        }
    }

    pub fn oracleRequest(&mut self, _sender: AccountId, _payment: U128, _specId: Base64String, _callbackAddress: AccountId, _callbackFunctionId: Base64String, _nonce: U128, _dataVersion: U128, _data: Base64String) {
        self.onlyLINK();
        self.checkCallbackAddress(&_callbackAddress);

        let payment_u128: u128 = _payment.into();
        let nonce_u128: u128 = _nonce.into();
        let dataVersion_u128: u128 = _dataVersion.into();

        let requestId: Base64String = hex::encode(env::keccak256(_sender, nonce_u128));

        let commitment_option = self.commitments.get(&requestId);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment == "", "Must use a unique ID");
        let expiration: u128 = env::block_timestamp() + EXPIRY_TIME;

        let to_insert: Base64String = hex::encode(env::keccak256(payment_u128, _callbackAddress, _callbackFunctionId, expiration));
        self.commitments.insert(&requestId, &to_insert);
    }

    pub fn fulfillOracleRequest(&mut self, _requestId: Base64String, _payment: U128, _callbackAddress: AccountId, _callbackFunctionId: Base64String, _expiration: U128, _data: Base64String) -> bool {
        self.isValidRequest(_requestId);
        let payment_u128: u128 = _payment.into();
        let expiration_u128: u128 = _expiration.into();

        let paramsHash: Base64String = hex::encode(env::keccak256(payment_u128, _callbackAddress, _callbackFunctionId, expiration_u128));
        assert!(self.commitments[_requestId] == paramsHash, "Params do not match request ID");
        self.withdrawableTokens = self.withdrawableTokens + _payment;
        self.commitments[_requestId].clear();
        // gasleft

        //call back
    }

    pub fn getAuthorizationStatus(&self, _node: AccountId) -> bool {
        self.authorizedNodes[_node]
    }

    pub fn setFulfillmentPermission(&mut self, _node: AccountId, _allowed: bool) {
        self.onlyOwner();

        self.authorizedNodes[_node] = _allowed;
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

    pub fn cancelOracleRequest(&mut self, _requestId: Base64String, _payment: U128, _callbackFunc: Base64String, _expiration: U128) {
        let payment_u128: u128 = _payment.into();
        let expiration_u128: u128 = _expiration.into();
        let paramsHash: Base64String = hex::encode(env::keccak256(payment_u128, _callbackAddress, _callbackFunctionId, expiration_u128));
        assert!(paramsHash == self.commitments[_requestId], "Params do not match request ID");
        assert!(expiration_u128 <= env::block_timestamp().into());

        let commitment_option = self.commitments.get(&_requestId);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        self.commitments.remove(&_requestId);

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

    fn isValidRequest(&self, _requestId: Base64String) {
        let commitment_option = self.commitments.get(&_requestId);
        if commitment_option.is_none() {
            env::panic(b"Could not find commitment.");
        }
        let commitment = commitment_option.unwrap();
        assert!(commitment != "", "Must have a valid requestId");
    }

    fn onlyAuthorizedNode(&self) {
        let node_option = self.authorizedNodes.get(&env::predecessor_account_id());
        assert!(node_option.is_some() || env::predecessor_account_id() == self.owner, "Not an authorized node to fulfill requests");
    }

    fn checkCallbackAddress(&self, _to: &AccountId) {
        assert_ne!(callback_address, &self.linkToken, "Cannot callback to LINK.");
        assert_ne!(callback_address, &env::current_account_id(), "Callback address cannot be the oracle contract.");
    }

    fn onlyOwner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }
}
