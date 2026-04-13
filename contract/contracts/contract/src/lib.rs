#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// Define the structure of a Maintenance Proposal
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub description: String,
    pub amount: i128,
    pub votes: u32,
    pub executed: bool,
}

#[contract]
pub struct MaintenanceDAO;

// Keys for storage
const PROPOSAL_COUNT: &str = "PROPOSAL_COUNT";

#[contractimpl]
impl MaintenanceDAO {
    
    /// Creates a new maintenance proposal.
    pub fn create_proposal(env: Env, proposer: Address, description: String, amount: i128) -> u32 {
        // Ensure the transaction is authorized by the proposer
        proposer.require_auth();

        // Get current proposal count or default to 0
        let mut count: u32 = env.storage().instance().get(&PROPOSAL_COUNT).unwrap_or(0);
        count += 1;

        // Construct the proposal
        let proposal = Proposal {
            id: count,
            proposer,
            description,
            amount,
            votes: 0,
            executed: false,
        };

        // Save the proposal to persistent storage and update the count
        env.storage().persistent().set(&count, &proposal);
        env.storage().instance().set(&PROPOSAL_COUNT, &count);

        count
    }

    /// Cast a vote for a specific proposal.
    pub fn vote(env: Env, voter: Address, proposal_id: u32) {
        // Ensure the voter authorizes the transaction
        voter.require_auth();

        // Retrieve the proposal from storage
        let mut proposal: Proposal = env.storage().persistent().get(&proposal_id).expect("Proposal not found");
        
        // Increment the vote count 
        // (Production note: Map voter addresses to proposal IDs to prevent double voting)
        proposal.votes += 1;
        
        // Save the updated proposal
        env.storage().persistent().set(&proposal_id, &proposal);
    }

    /// Execute the proposal if it has met the voting threshold.
    pub fn execute_proposal(env: Env, executor: Address, proposal_id: u32) {
        executor.require_auth();
        
        let mut proposal: Proposal = env.storage().persistent().get(&proposal_id).expect("Proposal not found");
        
        assert!(!proposal.executed, "Proposal has already been executed.");
        assert!(proposal.votes >= 5, "Proposal has not met the required vote threshold."); // Arbitrary threshold of 5 for demonstration
        
        // Mark as executed
        proposal.executed = true;
        env.storage().persistent().set(&proposal_id, &proposal);
        
        // TODO: Integrate Stellar token transfer logic here to disburse `proposal.amount` 
        // to a designated maintenance contractor address.
    }
}