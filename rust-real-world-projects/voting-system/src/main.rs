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


// Voter structure
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Voter {
    has_voted: HashMap<u64, bool>, // Proposal ID -> Has voted?
}

#[near_bindgen]
impl VotingContract {
    #[init]
    pub fn new(admin: AccountId) -> Self {
        Self {
            proposals: HashMap::new(),
            voters: HashMap::new(),
            admin,
            next_proposal_id: 0,
        }
    }

    // Function to create a new proposal
    pub fn create_proposal(&mut self, title: String, description: String, voting_duration_sec: u64) {
        let creator = env::predecessor_account_id();
        let proposal_id = self.next_proposal_id;

        let end_time = env::block_timestamp() / 1_000_000 + voting_duration_sec;

        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            votes_for: 0,
            votes_against: 0,
            creator: creator.clone(),
            is_open: true,
            end_time,
        };

        self.proposals.insert(proposal_id, proposal);
        self.next_proposal_id += 1;

        env::log_str(&format!("Proposal {} created by {}", proposal_id, creator));
    }

    // Vote on a proposal
    pub fn vote(&mut self, proposal_id: u64, support: bool) {
        let voter = env::predecessor_account_id();

        // Ensure proposal exists
        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .expect("Proposal not found");

        // Check if voting is still open
        assert!(proposal.is_open, "Voting for this proposal is closed.");
        assert!(
            env::block_timestamp() / 1_000_000 <= proposal.end_time,
            "Voting period has ended."
        );

        // Ensure the voter has not already voted on this proposal
        let voter_data = self.voters.entry(voter.clone()).or_default();
        assert!(
            !*voter_data.has_voted.entry(proposal_id).or_insert(false),
            "You have already voted on this proposal."
        );

        // Record the vote
        if support {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }
        voter_data.has_voted.insert(proposal_id, true);

        env::log_str(&format!(
            "Vote recorded for proposal {} by {}",
            proposal_id, voter
        ));
    }

    // Close a proposal (admin only)
    pub fn close_proposal(&mut self, proposal_id: u64) {
        let caller = env::predecessor_account_id();
        assert_eq!(caller, self.admin, "Only the admin can close proposals.");

        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .expect("Proposal not found");
        proposal.is_open = false;

        env::log_str(&format!("Proposal {} closed by admin.", proposal_id));
    }

    // View proposal details
    pub fn get_proposal(&self, proposal_id: u64) -> Proposal {
        self.proposals
            .get(&proposal_id)
            .expect("Proposal not found")
            .clone()
    }

    // Get a list of all proposals
    pub fn get_all_proposals(&self) -> Vec<Proposal> {
        self.proposals.values().cloned().collect()
    }

    // View voter details
    pub fn get_voter(&self, account_id: AccountId) -> Voter {
        self.voters
            .get(&account_id)
            .unwrap_or_else(|| panic!("Voter not found"))
            .clone()
    }
}