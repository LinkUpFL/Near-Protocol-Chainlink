use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;

pub type Base64String = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Request {
    id: Base64String,
    callbackAddress: AccountId,
    callbackFunctionId: Base64String,
    nonce: u256,
    // buffer
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Chainlink {
    defaultBufferSize: u256 = 256
}

#[near_bindgen]
impl Chainlink {
    fn initialize(&mut self, _self: Request, _id: Base64String, _callbackAddress: AccountId, _callbackFunction: Base64String) -> (Chainlink.Request) {
        //buffer
        _self.id = _id;
        _self.callbackAddress = _callbackAddress;
        _self.callbackFunctionId = _callbackFunction;
        return _self;
    }
}
