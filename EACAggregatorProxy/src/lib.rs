use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::wee_alloc::{WeeAlloc};
use std::str;
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
    pub proposedAggregator: AccountId,
    pub phaseAggregators: LookupMap<u64, AccountId>,
    pub accessController: AccountId,
    pub checkEnabled: bool,
    accessList: LookupMap<AccountId, bool>,
    currentPhase: Phase
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
            accessController: _accessController,
            proposedAggregator: _aggregator
        };

        result.checkEnabled = true;
        // result.setAggregator(&_aggregator);
        // result.setController(&_accessController);
        result
    }

    pub fn setController(&mut self, _accessController: AccountId) {
        self.onlyOwner();
        self.accessController = _accessController;
    }

    pub fn latestAnswer(&self) -> u128 {
        self.checkAccess();
        self.currentPhase.aggregator.latestAnswer()
    }

    pub fn latestTimestamp(&self) -> u128 {
        self.checkAccess();
        self.currentPhase.aggregator.latestTimestamp()
    }

    pub fn getAnswer(&mut self, _roundId: U128) -> u128 {
        self.checkAccess();

        let roundId_u128: u128 = _roundId.into();
        if roundId_u128 > self.MAX_ID {
            return 0;
        }

        let (phaseId, aggregatorRoundId): (u64, u64) = self.parseIds(roundId_u128);
        let aggregator: AccountId = self.phaseAggregators[phaseId];
        if aggregator == "" {
            return 0;
        } 

        return aggregator.getAnswer(aggregatorRoundId);
    }

    pub fn getTimestamp(&self, _roundId: U128) -> u128 {
        self.checkAccess();
        let roundId_u128: u128 = _roundId.into();
        if roundId_u128 > self.MAX_ID {
            return 0;
        }

        let (phaseId, aggregatorRoundId): (u64, u64) = self.parseIds(roundId_u128);
        let aggregator: AccountId = self.phaseAggregators[phaseId];
        if aggregator == "" {
            return 0;
        }

        return aggregator.getTimestamp(aggregatorRoundId);
    }

    pub fn latestRound(&mut self) -> (u128) {
        self.checkAccess();
        let phase: Phase = self.currentPhase;
        self.addPhase(phase.id, phase.aggregator.latestRound() as u64)
    }

    pub fn getRoundData(&mut self, _roundId: U128) -> (u128, u128, u128, u128, u64) {
        let roundId_u128: u128 = _roundId.into();
        let (phaseId, aggregatorRoundId): (u64, u64) = self.parseIds(roundId_u128);

        (self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound) = self.phaseAggregators[phaseId].getRoundData(aggregatorRoundId);

        return self.addPhaseIds(self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound, self.phaseId);
    }

    pub fn latestRoundData(&mut self) -> (u128, u128, u128, u128, u128) {
        let current: Phase = self.currentPhase; // cache storage reads

        (self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound) = current.aggregator.latestRoundata();

        return self.addPhaseIds(self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound, self.phaseId);
    }

    pub fn proposedGetRoundData(&self, _roundId: U128) -> (u128, u128, u128, u128, u128) {
        self.checkAccess();
        self.hasProposal();
        
        let roundId_u128: u128 = _roundId.into();
        self.proposedAggregator.getRoundData(roundId_u128)
    }

    pub fn proposedLatestRoundData(&self) -> (u128, u128, u128, u128,  u128) {
        self.checkAccess();
        self.hasProposal();
        self.proposedAggregator.latestRoundData()
    }

    pub fn aggregator(&self) -> AccountId {
        self.currentPhase.aggregator as AccountId
    }

    pub fn phaseId(&self) -> u64 {
        self.currentPhase.id
    }

    pub fn decimals(&self) -> u64 {
        self.currentPhase.aggregator.decimals();
    }

    pub fn version(&self) -> u128 {
        self.currentPhase.aggregator.version()
    }

    pub fn description(&self) -> Base64String {
        self.currentPhase.aggregator.description()
    }

    pub fn proposeAggregator(&mut self, _aggregator: AccountId) {
        self.onlyOwner();
        self.proposeAggregator = _aggregator;
    }

    pub fn confirmAggregator(&mut self, _aggregator: AccountId) {
        self.onlyOwner();
        assert!(_aggregator == self.proposedAggregator as AccountId, "Invalid proposed aggregator");
        self.proposeAggregator.clear();
        self.setAggregator(_aggregator);
    }

    // Internal

    fn setAggregator(&mut self, _aggregator: AccountId) {
        let id: u64 = self.currentPhase.id + 1;
        self.currentPhase = self.Phase(id, _aggregator);
        self.phaseAggregators[id] = _aggregator;
    }

    fn addPhase(&self, _phase: u64, _originalId: u64) -> u128 {
        ((_phase as u128) << self.PHASE_OFFSET | _originalId) as u128
    }

    fn parseIds(&self, _roundId: u128) -> (u64, u64) {
        let phaseId: u64 = (_roundId >> self.PHASE_OFFSET) as u64;
        let aggregatorRoundId: u64 = _roundId as u64;

        return(phaseId, aggregatorRoundId);
    }

    fn addPhaseIds(&self, roundId: u128, answer: u128, startedAt: u128, updatedAt: u128, answeredInRound: u128, phaseId: u64) -> (u128, u128, u128, u128, u128) {
        return(self.addPhase(phaseId, roundId as u64), answer, startedAt, updatedAt, self.addPhase(phaseId, answeredInRound as u64));
    }

    // Modifiers

    fn hasProposal(&mut self) {
        assert!(self.proposedAggregator != "", "No proposed aggregator present");
    }

    fn onlyOwner(&mut self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
    }

    // Access Control

    pub fn hasAccess(&self, _user: AccountId) -> bool {
        if !self.checkEnabled {
            !self.checkEnabled
        } else {
            let user_option = self.accessList.get(&_user);
            if user_option.is_none() {
                env::panic(b"Did not find this oracle account.");
            }
            let user = user_option.unwrap();
            user
        }
    }

    pub fn addAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        let user_option = self.accessList.get(&_user);
        if user_option.is_none() {
            let user = user_option.unwrap();
            user = true;
            env::panic(b"Added access to this oracle account.");
        }
    }

    pub fn removeAccess(&mut self, _user: AccountId) {
        self.onlyOwner();

        let user_option = self.accessList.get(&_user);
        if user_option.is_none() {
            env::panic(b"Did not find the oracle account to remove.");
        }
        let user = user_option.unwrap();
        user = false;
    }

    pub fn enableAccessCheck(&mut self) {
        self.onlyOwner();

        if !self.checkEnabled {
            self.checkEnabled = true;
        }
    }

    pub fn disableAccessCheck(&mut self) {
        self.onlyOwner();

        if self.checkEnabled {
            self.checkEnabled = false;
        }
    }

    fn checkAccess(&self) {
        let ac: AccountId = self.accessController;
        assert!(env::is_valid_account_id(ac.as_bytes()), "AC's account ID is invalid");
        // Check this since it's supposed to be calling hasAccess() from withitn this
        // contract called ac
        assert!(ac.hasAccess(env::predecessor_account_id()), "No access");
    }
}
