use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::wee_alloc::WeeAlloc;
use near_sdk::{env, near_bindgen, AccountId, Promise, PromiseOrValue, PromiseResult};
use num_traits::pow;
use std::str;
use std::convert::TryInto;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const SINGLE_CALL_GAS: u64 = 50_000_000_000_000; // 5 x 10^13

pub type Base64String = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Phase {
    id: u64,
    aggregator: AccountId,
}

const PHASE_OFFSET: u128 = 64;
// const PHASE_SIZE: u128 = 16;

fn find_pow() -> u128 {
    (pow(2, 80)) - 1
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EACAggregatorProxy {
    pub owner: AccountId,
    pub proposed_aggregator: AccountId,
    pub phase_aggregators: LookupMap<u64, AccountId>,
    pub access_controller: AccountId,
    current_phase: Phase,
}

impl Default for EACAggregatorProxy {
    fn default() -> Self {
        panic!("EACAggregatorProxy should be initialized before usage")
    }
}

#[near_bindgen]
impl EACAggregatorProxy {
    #[init]
    pub fn new(
        owner_id: AccountId,
        _aggregator: AccountId,
        _access_controller: AccountId,
    ) -> Self {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid"
        );
        assert!(!env::state_exists(), "Already initialized");

        let mut result = Self {
            owner: owner_id,
            proposed_aggregator: "".to_string(),
            phase_aggregators: LookupMap::new(b"phase_aggregators".to_vec()),
            access_controller: "".to_string(),
            current_phase: Phase {
                id: 0_u64,
                aggregator: "".to_string(),
            },
        };

        result.set_aggregator(_aggregator);
        result.set_controller(_access_controller);
        result
    }

    pub fn set_controller(&mut self, _access_controller: AccountId) {
        self.only_owner();
        self.access_controller = _access_controller;
    }
    // Depracated
    
    pub fn latest_answer(&mut self) {
        self.check_access();
        let prepaid_gas = env::prepaid_gas();
        let get_latest_answer_promise = env::promise_create(
            self.current_phase.aggregator.clone(),
            b"latest_answer",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );

        let promise3 = env::promise_then(get_latest_answer_promise, env::current_account_id(), b"latest_answer_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
        env::promise_return(promise3);
    }

    pub fn latest_answer_results(&self) -> u128 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_latest_answer_promise_result: Vec<u8> =
        match env::promise_result(0) {
            PromiseResult::Successful(_x) => {
                env::log(b"Check_promise successful");
                _x
            }
            _x => panic!("Promise with index 0 failed"),
        };
        serde_json::from_slice(&get_latest_answer_promise_result).unwrap()
    }
    // Depracated

    pub fn latest_timestamp(&mut self) {
        self.check_access();
        let prepaid_gas = env::prepaid_gas();
        let get_latest_timestamp_promise = env::promise_create(
            self.current_phase.aggregator.clone(),
            b"latest_timestamp",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );

        let promise3 = env::promise_then(get_latest_timestamp_promise, env::current_account_id(), b"latest_timestamp_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
        env::promise_return(promise3);
    }

    pub fn latest_timestamp_results(&self) -> u64 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_latest_timestamp_promise_result: Vec<u8> =
        match env::promise_result(0) {
            PromiseResult::Successful(_x) => {
                env::log(b"Check_promise successful");
                _x
            }
            _x => panic!("Promise with index 0 failed"),
        };
        serde_json::from_slice(&get_latest_timestamp_promise_result).unwrap()
    }

    // Depracated
    // pub fn get_answer(&mut self, _round_id: U128) -> PromiseOrValue<u128> {
    //     self.check_access();
    //     let prepaid_gas = env::prepaid_gas();
    //     let round_id_u128: u128 = _round_id.into();
    //     if round_id_u128 > find_pow() {
    //         return PromiseOrValue::Value(0);
    //     }

    //     let (phase_id, aggregator_round_id): (u64, u64) = self.parse_ids(round_id_u128);

    //     let aggregator_option = self.phase_aggregators.get(&phase_id);
    //     if aggregator_option.is_none() {
    //         env::panic(b"Aggregator account not found");
    //     }
    //     let phase_aggregator = aggregator_option.unwrap();
    //     let get_latest_timestamp_promise = env::promise_create(
    //         self.current_phase.aggregator.clone(),
    //         b"get_answer",
    //         json!({"_roundId": aggregator_round_id}).to_string().as_bytes(),
    //         0,
    //         SINGLE_CALL_GAS,
    //     );

    //     let promise3 = env::promise_then(get_latest_timestamp_promise, env::current_account_id(), b"latest_timestamp_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
    //     env::promise_return(promise3)
        
    // }

    // pub fn get_answer_results(&self) -> u128 {
    //     assert_eq!(env::current_account_id(), env::predecessor_account_id());
    //     assert_eq!(env::promise_results_count(), 1);
    //     let get_answer_promise_result: Vec<u8> =
    //     match env::promise_result(0) {
    //         PromiseResult::Successful(_x) => {
    //             env::log(b"Check_promise successful");
    //             _x
    //         }
    //         _x => panic!("Promise with index 0 failed"),
    //     };
    //     serde_json::from_slice(&get_answer_promise_result).unwrap()
    // }
    // Depracated
    // pub fn get_timestamp(&self, _round_id: U128) -> PromiseOrValue<u128> {
    //     self.check_access();
    //     let round_id_u128: u128 = _round_id.into();
    //     if round_id_u128 > find_pow() {
    //         return PromiseOrValue::Value(0);
    //     }

    //     let (phase_id, aggregator_round_id): (u64, u64) = self.parse_ids(round_id_u128);

    //     let aggregator_option = self.phase_aggregators.get(&phase_id);
    //     if aggregator_option.is_none() {
    //         env::panic(b"Aggregator account not found");
    //     }
    //     let phase_aggregator = aggregator_option.unwrap();

    //     PromiseOrValue::Promise(
    //         Promise::new(phase_aggregator)
    //             .function_call(
    //                 b"get_timestamp".to_vec(),
    //                 json!({ "_roundId": aggregator_round_id })
    //                     .to_string()
    //                     .as_bytes()
    //                     .to_vec(),
    //                 0,
    //                 SINGLE_CALL_GAS,
    //             )
    //             .as_return(),
    //     )
    // }

    pub fn latest_round(&mut self) {
        self.check_access();
        let prepaid_gas = env::prepaid_gas();
        let get_latest_round_promise = env::promise_create(
            self.current_phase.aggregator.clone(),
            b"latest_round",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );

        let promise3 = env::promise_then(get_latest_round_promise, env::current_account_id(), b"latest_round_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
        env::promise_return(promise3);
    }

    pub fn latest_round_results(&self) -> u128 {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_latest_round_promise_result: Vec<u8> =
        match env::promise_result(0) {
            PromiseResult::Successful(_x) => {
                env::log(b"Check_promise successful");
                _x
            }
            _x => panic!("Promise with index 0 failed"),
        };
        let latest_round_id: u64 = serde_json::from_slice(&get_latest_round_promise_result).unwrap();
        self.add_phase(self.current_phase.id, latest_round_id.try_into().unwrap())
    }

    pub fn get_round_data(&mut self, _round_id: U128)  {
        self.check_access();
        let prepaid_gas = env::prepaid_gas();
        let round_id_u128: u128 = _round_id.into();
        let (phase_id, aggregator_round_id): (u64, u64) = self.parse_ids(round_id_u128);
        let phase_aggregator_option = self.phase_aggregators.get(&phase_id);
        if phase_aggregator_option.is_none() {
            env::panic(b"Phase aggregator account not found");
        }
        let phase_aggregator_option_address = phase_aggregator_option.unwrap();
        let get_round_data_promise = env::promise_create(
            phase_aggregator_option_address,
            b"get_round_data",
            json!({ "_round_id": aggregator_round_id })
                .to_string()
                .as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let promise3 = env::promise_then(get_round_data_promise, env::current_account_id(), b"get_round_data_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
        env::promise_return(promise3);
       
    }
    pub fn get_round_data_results(&self) -> (u128, u128, u128, u128, u128) {
        let get_round_data_promise_result: Vec<u8> =
        match env::promise_result(0) {
            PromiseResult::Successful(_x) => {
                env::log(b"Check_promise successful");
                _x
            }
            _x => panic!("Promise with index 0 failed"),
        };
        let get_round_data_promise_result_json: (u128, u128, u128, u128, u64) =
        serde_json::from_slice(&get_round_data_promise_result).unwrap();
    self.add_phase_ids(
        get_round_data_promise_result_json.0,
        get_round_data_promise_result_json.0,
        get_round_data_promise_result_json.1,
        get_round_data_promise_result_json.2,
        get_round_data_promise_result_json.3,
        get_round_data_promise_result_json.4,
    )
    }

    pub fn latest_round_data(&mut self)  {
        // let current = &self.current_phase; // cache storage reads
        self.check_access();
        let prepaid_gas = env::prepaid_gas();
        let get_latest_round_data_promise = env::promise_create(
            self.current_phase.aggregator.clone(),
            b"latest_round_data",
            json!({}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        let promise3 = env::promise_then(get_latest_round_data_promise, env::current_account_id(), b"latest_round_data_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
        env::promise_return(promise3);
    }

    pub fn latest_round_data_results(&self) -> (u128, u128, u128, u128, u128) {
        let get_latest_round_data_promise_result: Vec<u8> =
        match env::promise_result(0) {
            PromiseResult::Successful(x) => x,
            _ => panic!("Promise with index 0 failed"),
        };
    let get_latest_round_data_promise_result_json: (u128, u128, u128, u128, u64) =
        serde_json::from_slice(&get_latest_round_data_promise_result).unwrap();
    self.add_phase_ids(
        get_latest_round_data_promise_result_json.0,
        get_latest_round_data_promise_result_json.0,
        get_latest_round_data_promise_result_json.1,
        get_latest_round_data_promise_result_json.2,
        get_latest_round_data_promise_result_json.3,
        self.current_phase.id.clone(),
    )
    }

    pub fn proposed_get_round_data(&mut self, _round_id: U128) -> Promise {
        self.check_access();
        self.has_proposal();
        let round_id_u128: u128 = _round_id.into();
        Promise::new(self.proposed_aggregator.clone())
            .function_call(
                b"get_round_data".to_vec(),
                json!({ "_round_id": round_id_u128 })
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
    }

    pub fn proposed_latest_round_data(&mut self) -> Promise {
        self.check_access();
        self.has_proposal();
        Promise::new(self.proposed_aggregator.clone())
            .function_call(
                b"latest_round_data".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
    }

    pub fn aggregator(&self) -> String {
        self.current_phase.aggregator.clone()
    }
    
    pub fn phase_id(&self) -> u64 {
        self.current_phase.id
    }

    pub fn decimals(&self) -> Promise {
        Promise::new(self.current_phase.aggregator.clone())
            .function_call(
                b"get_decimals".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
    }

    pub fn version(&self) -> Promise {
        Promise::new(self.current_phase.aggregator.clone())
            .function_call(
                b"get_version".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
    }

    pub fn description(&self) -> Promise {
        Promise::new(self.current_phase.aggregator.clone())
            .function_call(
                b"get_description".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                SINGLE_CALL_GAS,
            )
            .as_return()
    }

    pub fn propose_aggregator(&mut self, _aggregator: AccountId) {
        self.only_owner();
        self.proposed_aggregator = _aggregator;
    }

    pub fn confirm_aggregator(&mut self, _aggregator: AccountId) {
        self.only_owner();
        assert!(
            _aggregator == self.proposed_aggregator.clone(),
            "Invalid proposed aggregator"
        );
        self.proposed_aggregator.clear();
        self.set_aggregator(_aggregator);
    }

    // Internal

    fn set_aggregator(&mut self, _aggregator: AccountId) {
        let id: u64 = self.current_phase.id.saturating_add(1);
        let phase_aggregator_option = self.phase_aggregators.get(&id);
        if phase_aggregator_option.is_some() {
            env::panic(b"Phase aggregator account not found");
        }
        self.phase_aggregators.insert(&id, &_aggregator);
        self.current_phase = Phase {id: id, aggregator: _aggregator};
    }

    fn add_phase(&self, _phase: u64, _original_id: u64) -> u128 {
        (_phase as u128) << PHASE_OFFSET | _original_id as u128
    }

    fn parse_ids(&self, _round_id: u128) -> (u64, u64) {
        let phase_id: u64 = (_round_id >> PHASE_OFFSET) as u64;
        let aggregator_round_id: u64 = _round_id as u64;

        (phase_id, aggregator_round_id)
    }

    fn add_phase_ids(
        &self,
        round_id: u128,
        answer: u128,
        started_at: u128,
        updated_at: u128,
        answered_in_round: u128,
        phase_id: u64,
    ) -> (u128, u128, u128, u128, u128) {
        (
            self.add_phase(phase_id, round_id as u64),
            answer,
            started_at,
            updated_at,
            self.add_phase(phase_id, answered_in_round as u64),
        )
    }

    // Modifiers

    fn has_proposal(&mut self) {
        assert!(
            self.proposed_aggregator != "",
            "No proposed aggregator present"
        );
    }

    fn only_owner(&mut self) {
        assert_eq!(
            self.owner,
            env::predecessor_account_id(),
            "Only contract owner can call this method."
        );
    }



    fn call_access_controller_has_access(&mut self, _user: AccountId) {
        let prepaid_gas = env::prepaid_gas();
        let get_has_access_promise = env::promise_create(
            self.access_controller.clone(),
            b"has_access",
            json!({"_user": _user}).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );

        let promise3 = env::promise_then(get_has_access_promise, env::current_account_id(), b"call_access_controller_has_access_results", json!({}).to_string().as_bytes(), 0, prepaid_gas / 4);
        env::promise_return(promise3);
    }

    fn call_access_controller_has_access_results(&self) -> bool {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        assert_eq!(env::promise_results_count(), 1);
        let get_has_access_promise_result: Vec<u8> =
        match env::promise_result(0) {
            PromiseResult::Successful(_x) => {
                env::log(b"Check_promise successful");
                _x
            }
            _x => panic!("Promise with index 0 failed"),
        };
        serde_json::from_slice(&get_has_access_promise_result).unwrap()
    }
    fn check_access(&self) {
        //  || self.call_access_controller_has_access(env::predecessor_account_id()
        assert!(
            self.access_controller == "null",
            "No access"
        );

    }

}

  