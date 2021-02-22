use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, Promise};
use near_sdk::wee_alloc::{WeeAlloc};
use std::str;
use std::convert::TryInto;
use num_traits::pow;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub type Base64String = String;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Phase {
    id: u64,
    aggregator: AccountId
}

const PHASE_OFFSET: u128 = 64;
const PHASE_SIZE: u128 = 16;
const MAX_ID: u128 = pow(PHASE_OFFSET+PHASE_SIZE, 2) - 1;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct EACAggregatorProxy {
    pub owner: AccountId,
    pub proposed_aggregator: AccountId,
    pub phase_aggregators: LookupMap<u64, Phase>,
    pub access_controller: AccountId,
    pub check_enabled: bool,
    access_list: LookupMap<AccountId, bool>,
    current_phase: Phase
}

impl Default for EACAggregatorProxy {
    fn default() -> Self {
        panic!("EACAggregatorProxy should be initialized before usage")
    }
}

#[near_bindgen]
impl EACAggregatorProxy {
    #[init]
    pub fn new(link_id: AccountId, owner_id: AccountId, _aggregator: AccountId, _accessController: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        assert!(env::is_valid_account_id(link_id.as_bytes()), "Link token account ID is invalid");
        assert!(!env::state_exists(), "Already initialized");

        let mut result = Self {
            owner: owner_id,
            proposed_aggregator: "".to_string(),
            phase_aggregators: LookupMap::new(b"phase_aggregators".to_vec()),
            access_controller: "".to_string(),
            check_enabled: true,
            access_list: LookupMap::new(b"access_list".to_vec()),
            current_phase: Phase { id: 0_u64, aggregator: "".to_string() }
        };

        result.set_aggregator(_aggregator);
        result.set_controller(_accessController);
        result
    }

    pub fn set_controller(&mut self, _access_controller: AccountId) {
        self.only_owner();
        self.access_controller = _access_controller;
    }
    // Depracated
    pub fn latest_answer(&self) -> Promise {
        self.check_access();
        Promise::new(self.current_phase.aggregator)
            .function_call(
                b"latest_answer".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
        }
    // Depracated
    pub fn latest_timestamp(&self) -> Promise {
        self.check_access();
        Promise::new(self.current_phase.aggregator)
            .function_call(
                b"lastest_timestamp".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
        }
    // Depracated
    pub fn get_answer(&mut self, _round_id: U128) -> Promise {
             self.check_access();
            let round_id_u128: u128 = _round_id.into();
            if round_id_u128 > MAX_ID {
                return 0;
            }
    
            let (phase_id, aggregator_round_id): (u64, u64) = self.parse_ids(round_id_u128);
    
            let aggregator_option = self.phase_aggregators.get(&phase_id);
            if aggregator_option.is_none() {
                env::panic(b"Aggregator account not found");
            }
            let phase_aggregator = aggregator_option.unwrap();
    
            if phase_aggregator.aggregator == "" {
                return 0;
            }
            Promise::new(phase_aggregator.aggregator)
                .function_call(
                    b"get_answer".to_vec(),
                    json!({"_roundId": aggregator_round_id}).to_string().as_bytes().to_vec(),
                    0,
                    SINGLE_CALL_GAS,
                )
                .as_return()
            }
    // Depracated
    pub fn get_timestamp(&self, _roundId: U128) -> u128 {
        self.check_access();
        let round_id_u128: u128 = _round_id.into();
        if round_id_u128 > MAX_ID {
            return 0;
        }

        let (phase_id, aggregator_round_id): (u64, u64) = self.parse_ids(round_id_u128);

        let aggregator_option = self.phase_aggregators.get(&phase_id);
        if aggregator_option.is_none() {
            env::panic(b"Aggregator account not found");
        }
        let phase_aggregator = aggregator_option.unwrap();

        if phase_aggregator.aggregator == "" {
            return 0;
        }
        Promise::new(phase_aggregator.aggregator)
            .function_call(
                b"get_timestamp".to_vec(),
                json!({"_roundId": aggregator_round_id}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
        }

    pub fn latest_round(&mut self) -> u128 {
        self.check_access();
        let phase: Phase = self.current_phase;
        let round_id: u64 = Promise::new(phase.aggregator)
        .function_call(
            b"latest_round".to_vec(),
            json!({}).to_string().as_bytes().to_vec(),
            0,
            SINGLE_CALL_GAS,
        )
        .as_return();
        self.add_phase(phase.id, round_id);
        round_id
    }

    pub fn get_round_data(&mut self, _round_id: U128) -> (u128, u128, u128, u128, u64) {

        let round_id_u128: u128 = _round_id.into();
        let (phase_id, aggregator_round_id): (u64, u64) = self.parse_ids(round_id_u128);
        let phase_aggregator_option = self.phase_aggregators.get(&id);
        if phase_aggregator_option.is_none() {
            env::panic(b"Phase aggregator account not found");
        }
        phase_aggregator = phase_aggregator_option.unwrap();
        (self.round_id, self.answer, self.started_at, self.updated_at, self.answered_in_round) = Promise::new(phase_aggregator)
        .function_call(
            b"get_round_data".to_vec(),
            json!({"_round_id": aggregator_round_id}).to_string().as_bytes().to_vec(),
            0,
            SINGLE_CALL_GAS,
        )
        .as_return();
        self.add_phase_ids(self.round_id, self.answer, self.started_at, self.updated_at, self.answered_in_round, phase_id);
    }

    pub fn latest_round_data(&mut self) -> (u128, u128, u128, u128, u128) {
        let current: Phase = self.current_phase; // cache storage reads

        (self.round_id, self.answer, self.started_at, self.updated_at, self.answered_in_round) = Promise::new(current.aggregator)
        .function_call(
            b"latest_round_data".to_vec(),
            json!().to_string().as_bytes().to_vec(),
            0,
            SINGLE_CALL_GAS,
        )
        .as_return();
        self.add_phase_ids(self.round_id, self.answer, self.started_at, self.updated_at, self.answered_in_round, phase_id)
    }

    pub fn proposed_get_round_data(&self, _round_id: U128) -> Promise {
        self.check_access();
        self.has_proposal();
        
        let round_id_u128: u128 = _round_id.into();
        Promise::new(self.proposed_aggregator)
        .function_call(
            b"get_round_data".to_vec(),
            json!({"_round_id": round_id_u128}).to_string().as_bytes().to_vec(),
            0,
            SINGLE_CALL_GAS,
        )
        .as_return()
    }

    pub fn proposed_latest_round_data(&self) -> Promise {
        self.check_access();
        self.has_proposal();
        Promise::new(self.proposed_aggregator)
        .function_call(
            b"latest_round_data".to_vec(),
            json!({}).to_string().as_bytes().to_vec(),
            0,
            SINGLE_CALL_GAS,
        )
        .as_return()    
    }

    pub fn aggregator(&self) -> AccountId {
        self.current_phase.aggregator as AccountId
    }

    pub fn phase_id(&self) -> u64 {
        self.current_phase.id
    }

    // pub fn decimals(&self) -> u64 {
    //     self.current_phase.aggregator.decimals()
    // }

    // pub fn version(&self) -> u128 {
    //     self.currentPhase.aggregator.version()
    // }

    // pub fn description(&self) -> Base64String {
    //     self.currentPhase.aggregator.description()
    // }

    pub fn propose_aggregator(&mut self, _aggregator: AccountId) {
        self.only_owner();
        self.proposed_aggregator = _aggregator;
    }

    pub fn confirm_aggregator(&mut self, _aggregator: AccountId) {
        self.only_owner();
        assert!(_aggregator == self.proposed_aggregator as AccountId, "Invalid proposed aggregator");
        self.proposed_aggregator.clear();
        self.set_aggregator(_aggregator);
    }

    // Internal

    fn set_aggregator(&mut self, _aggregator: AccountId) {
        let id: u64 = self.current_phase.id + 1;
        let phase_aggregator_option = self.phase_aggregators.get(&id);
        if phase_aggregator_option.is_none() {
            env::panic(b"Phase aggregator account not found");
        }
        let phase_aggregator = phase_aggregator_option.unwrap();
        self.current_phase = self.Phase(id, _aggregator);
        self.phase_aggregators.insert(&id, &_aggregator);
    }

    fn add_phase(&self, _phase: u64, _original_id: u64) -> u128 {
        ((_phase as u128) << PHASE_OFFSET | _original_id) as u128
    }

    fn parse_ids(&self, _round_id: u128) -> (u64, u64) {
        let phase_id: u64 = (_round_id >> PHASE_OFFSET) as u64;
        let aggregator_round_id: u64 = _round_id as u64;

        (phase_id, aggregator_round_id)
    }

    fn add_phase_ids(&self, round_id: u128, answer: u128, started_at: u128, updated_at: u128, answered_in_round: u128, phase_id: u64) -> (u128, u128, u128, u128, u128) {
        (self.add_phase(phase_id, round_id as u64), phase_id, answer, startedAt, updatedAt, answered_in_round, self.add_phase(phase_id, answered_in_round as u64))
    }

    // Modifiers

    fn has_proposal(&mut self) {
        assert!(self.proposed_aggregator != "", "No proposed aggregator present");
    }

    fn only_owner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }

    // Access Control

    pub fn has_access(&self, _user: AccountId) -> bool {
        if !self.check_enabled {
            !self.check_enabled
        } else {
            let user_option = self.access_list.get(&_user);
            if user_option.is_none() {
                env::panic(b"Did not find this oracle account.");
            }
            let user = user_option.unwrap();
            user
        }
    }

    pub fn add_access(&mut self, _user: AccountId) {
        self.only_owner();

        let user_option = self.access_list.get(&_user);
        if user_option.is_none() {
            self.access_list.insert(&_user, &true);
            env::panic(b"Added access to this oracle account.");
        }
    }

    pub fn remove_access(&mut self, _user: AccountId) {
        self.only_owner();

        let user_option = self.access_list.get(&_user);
        if user_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        self.access_list.insert(&_user, &false);
    }

    pub fn enable_access_check(&mut self) {
        self.only_owner();

        if !self.check_enabled {
            self.check_enabled = true;
        }
    }

    pub fn disable_access_check(&mut self) {
        self.only_owner();

        if self.check_enabled {
            self.check_enabled = false;
        }
    }

    fn check_access(&self) {
        let ac: AccountId = self.access_controller;
        assert!(env::is_valid_account_id(ac.as_bytes()), "AC's account ID is invalid");
        // Check this since it's supposed to be calling hasAccess() from withitn this
        // contract called ac
        assert!(ac.hasAccess(env::predecessor_account_id()), "No access");
    }
}
