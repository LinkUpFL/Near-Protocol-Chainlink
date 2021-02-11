use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use near_sdk::collections::{TreeMap, UnorderedSet, LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, env, near_bindgen, PromiseResult};
use serde_json::json;
use std::str;
use std::collections::HashMap;
use num_traits::pow;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type Base64String = String;

#[derive(Serialize, Deserialize)]
pub struct Phase {
    id: u16,
    aggregator: AccountId
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AggregatorProxy {
    currentPhase: Phase,
    pub proposedAggregator: AccountId,
    pub phaseAggregators: LookupMap<u16, AccountId>,
    PHASE_OFFSET: u256 = 64,
    PHASE_SIZE: u256 = 16,
    MAX_ID: u256 = 2.pow(PHASE_OFFSET+PHASE_SIZE) - 1;
}

#[near_bindgen]
impl AggregatorProxy {
    pub fn latestAnswer(&self) -> (answer: i256) {
        self.currentPhase.aggregator.latestAnswer()
    }

    pub fn latestTimestamp(&self) -> (updatedAt: u256) {
        self.currentPhase.aggregator.latestTimestamp()
    }

    pub fn getAnswer(&mut self, _roundId: U128) -> (answer: i256) {
        let roundId_u128: u128 = _roundId.into();
        if(roundId_u128 > self.MAX_ID) return 0;

        let (phaseId: u16, aggregatorRoundId: u64) = self.parseIds(roundId_u128);
        let aggregator: AccountId = self.phaseAggregators[phaseId];
        if(aggregator == "") return 0;

        return aggregator.getAnswer(aggregatorRoundId);
    }

    pub fn getTimestamp(&self, _roundId: U128) -> (updatedAt: u256) {
        let roundId_u128: u128 = _roundId.into();
        if(roundId_u128 > self.MAX_ID) return 0;

        let (phaseId: u16, aggregatorRoundId: u64) = self.parseIds(roundId_u128);
        let aggregator: AccountId = self.phaseAggregators[phaseId];
        if(aggregator == "") return 0;

        return aggregator.getTimestamp(aggregatorRoundId);
    }

    pub fn latestRound(&mut self) -> (roundId: u256) {
        let phase: Phase = self.currentPhase;
        self.addPhase(phase.id, phase.aggregator.latestRound() as u64)
    }

    pub fn getRoundData(&mut self, _roundId: U128) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        let roundId_u128: u128 = _roundId.into();
        let (phaseId: u16, aggregatorRoundId: u64) = self.parseIds(roundId_u128);

        (self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound) = self.phaseAggregators[phaseId].getRoundData(aggregatorRoundId);

        return self.addPhaseIds(self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound, self.phaseId);
    }

    pub fn latestRoundData(&mut self) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        let current: Phase = self.currentPhase; // cache storage reads

        (self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound) = current.aggregator.latestRoundData();

        return self.addPhaseIds(self.roundId, self.answer, self.startedAt, self.updatedAt, self.answeredInRound, self.phaseId);
    }

    pub fn proposedGetRoundData(&self, _roundId: U128) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        let roundId_u128: u128 = _roundId.into();
        self.hasProposal();
        self.proposedAggregator.getRoundData(roundId_u128)
    }

    pub fn proposedLatestRoundData(&self) -> (roundId: u80, answer: i256, startedAt: u256, updatedAt: u256, answeredInRound: u80) {
        self.hasProposal();
        self.proposedAggregator.latestRoundData()
    }

    pub fn aggregator(&self) -> AccountId {
        self.currentPhase.aggregator as AccountId
    }

    pub fn phaseId(&self) -> u16 {
        self.currentPhase.id
    }

    pub fn decimals(&self) -> u8 {
        self.currentPhase.aggregator.decimals();
    }

    pub fn version(&self) -> u256 {
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
        let id: u16 = self.currentPhase.id + 1;
        self.currentPhase = self.Phase(id, _aggregator);
        self.phaseAggregators[id] = _aggregator;
    }

    fn addPhase(&self, _phase: u64, _originalId: u128) -> u128 {
        (_phase << self.PHASE_OFFSET | _originalId) as u128
    }

    fn parseIds(&self, _roundId: u128) -> (u16, u128) {
        let phaseId: u16 = (_roundId >> self.PHASE_OFFSET) as u16;
        let aggregatorRoundId: u128 = _roundId;

        return(phaseId, aggregatorRoundId);
    }

    fn addPhaseIds(&self, roundId: u128, answer: u128, startedAt: u128, updatedAt: u128, answeredInRound: u128, phaseId: u128) -> (u80, i256, u256, u256, u80) {
        return(self.addPhase(phaseId, roundId), answer, startedAt, updatedAt, self.addPhase(phaseId, answeredInRound));
    }

    // Modifiers

    fn hasProposal(&mut self) {
        assert!(self.proposedAggregator != "", "No proposed aggregator present");
    }

    fn onlyOwner(&mut self) {
        assert_eq!(env::signer_account_id(), env::current_account_id(), "Only contract owner can call this method.");
    }
}
