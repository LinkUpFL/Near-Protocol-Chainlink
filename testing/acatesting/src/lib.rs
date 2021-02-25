use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use std::str;
use std::convert::TryInto;
use num_traits::pow;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub type Base64String = String;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Round {
    answer: u128,
    startedAt: u64,
    updatedAt: u64,
    answeredInRound: u64
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RoundDetails {
    submissions: Vec<u128>,
    maxSubmissions: u64,
    minSubmissions: u64,
    timeout: u64,
    paymentAmount: u128
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OracleStatus {
    withdrawable: u128,
    startingRound: u64,
    endingRound: u128,
    lastReportedRound: u64,
    lastStartedRound: u64,
    latestSubmission: u128,
    index: u64,
    admin: AccountId,
    pendingAdmin: AccountId
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Requester {
    authorized: bool,
    delay: u64,
    lastStartedRound: u64
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Funds {
    available: u128,
    allocated: u128
}

const version: u128 = 3;
const RESERVE_ROUNDS: u128 = 2;
const MAX_ORACLE_COUNT: u128 = 77;
// Previous: 2.pow(32-1)
// const ROUND_MAX: u128 = pow(32-1, 2);
// const V3_NO_DATA_ERROR: Base64String = "No data present".to_string();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AcaTesting {
    pub owner: AccountId,
    pub linkToken: AccountId,
    pub validator: AccountId,
    pub paymentAmount: u128,
    pub maxSubmissionCount: u64,
    pub minSubmissionCount: u64,
    pub restartDelay: u64,
    pub timeout: u64,
    pub decimals: u64,
    pub description: Base64String,
    pub minSubmissionValue: u128,
    pub maxSubmissionValue: u128,
    pub checkEnabled: bool,
    accessList: LookupMap<AccountId, bool>,
    reportingRoundId: u64,
    latestRoundId: u64,
    oracles: LookupMap<AccountId, OracleStatus>,
    rounds: LookupMap<u64, Round>,
    details: LookupMap<u128, RoundDetails>,
    requesters: LookupMap<AccountId, Requester>,
    oracleAddresses: Vec<AccountId>,
    recordedFunds: Funds
}

impl Default for AcaTesting {
    fn default() -> Self {
        panic!("AccessControlledAggregator should be initialized before usage");
    }
}

#[near_bindgen]
impl AcaTesting {
    #[init]
    pub fn new(link_id: AccountId, owner_id: AccountId, _timeout: U64, _validator: AccountId, _minSubmissionValue: U128, _maxSubmissionValue: U128, _decimals: U64, _description: String) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(link_id.as_bytes()), "Link token account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let minSubmissionValue_u128: u128 = _minSubmissionValue.into();
        let maxSubmissionValue_u128: u128 = _maxSubmissionValue.into();
        let decimals_u64: u64 = _decimals.into();
        let vector: Vec::<AccountId> = Vec::new();

        let mut result = Self {
            owner: owner_id,
            linkToken: link_id,
            validator: "".to_string(),
            paymentAmount: 0_u128,
            maxSubmissionCount: 0_u64,
            minSubmissionCount: 0_u64,
            restartDelay: 0_u64,
            timeout: 0_u64,
            decimals: decimals_u64,
            description: _description,
            minSubmissionValue: minSubmissionValue_u128,
            maxSubmissionValue: maxSubmissionValue_u128,
            checkEnabled: true,
            accessList: LookupMap::new(b"access_list".to_vec()),
            reportingRoundId: 0_u64,
            latestRoundId: 0_u64,
            oracles: LookupMap::new(b"oracles".to_vec()),
            rounds: LookupMap::new(b"rounds".to_vec()),
            details: LookupMap::new(b"details".to_vec()),
            requesters: LookupMap::new(b"requesters".to_vec()),
            oracleAddresses: vector,
            recordedFunds: Funds { available: 0_u128, allocated: 0_u128 }
        };
        result.checkEnabled = true;
        result
    }

    pub fn latest_round(&self) -> u64 {
        1
    }
}
