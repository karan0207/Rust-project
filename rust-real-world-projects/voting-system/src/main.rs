use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use std::collections::HashMap;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct VotingContract {
    proposals: HashMap<u64, Proposal>,
    voters: HashMap<AccountId, Voter>,
    admin: AccountId,
    next_proposal_id: u64,
}