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


// Proposal structure
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal {
    id: u64,
    title: String,
    description: String,
    votes_for: u32,
    votes_against: u32,
    creator: AccountId,
    is_open: bool,
    end_time: u64, // UNIX timestamp for when voting ends
}