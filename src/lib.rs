// Start
// |
// v
// Define Data Structures
// |
// v
// Define Contract Functions
// |
// v
// Define Initialization Logic
// |
// v
// Is
// Security
// Important? 
// |   |
// |   v
// Yes  No
// |   |
// |   v
// Define Security Mechanisms
// |   |
// |   v
// Implement Access Control
// |
// v
// Define Share Transfer Logic
// |
// v
// Define Proposal Creation Logic
// |
// v
// Define Proposal Execution Logic
// |
// v
// Implement Voting Mechanism
// |
// v
// Enforce Contract Rules
// |
// v
// Write Utility Functions
// |
// v
// Unit Testing & Debugging
// |
// v
// End - Contract Complete





#![no_std]

// Import necessary libraries
use core::{panic};

use soroban_sdk::{
    contractimpl, contracttype, symbol, Address, BytesN, ConversionError,
    Env, RawVal, Symbol, TryFromVal, Vec,
};

// Import the 'token' module from an external file
mod token {
    soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}


// Define a contract type named 'ProposalVote' with two fields: 'voter' and 'prop_id'
#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalVote {
    pub voter: Address,
    pub prop_id: u32,
}

// Define an enum named 'DataKey' to represent various keys used for contract data
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TotSupply,
    Balance(Address),
    Bootstrap,
    Proposal(u32),
    ProposalId,
    Voted(ProposalVote),
    Executed(u32)
}

// Define a contract type named 'ProposalInstr' with fields: 'c_id', 'fun_name', and 'args'
#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalInstr {
    // Contract ID
    pub c_id: BytesN<32>,
    pub fun_name: Symbol,
    pub args: Vec<RawVal>,
}



// Define a contract type named 'Proposal' with fields: 'tot_votes', 'end_time', and 'instr'
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub tot_votes: i32,
    pub end_time: u64,
    // Instructions will be executed in sequence
    pub instr: Vec<ProposalInstr>,
}



// Define a trait named 'DaoTrait' with various contract functions
pub trait DaoTrait {
    fn init(env: Env);
    fn x_shares(env: Env, amount: i32, to: Address);
    fn c_prop(env: Env, proposal: Proposal) -> u32;
    fn execute(env: Env, prop_id: u32);
    fn shares(env: Env, of: Address) -> i32;
    fn tot_shares(env: Env) -> i32;
    fn vote(env: Env, prop_id: u32);
}

// Define the 'DaoContract' struct, which will implement the 'DaoTrait' contract functions
pub struct DaoContract;
