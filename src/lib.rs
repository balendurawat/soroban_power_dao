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
    // fn test(env : Env) -> Address;
    fn init(env: Env);
    // transfer sahres if admin and if in bootstrap period
    fn x_shares(env: Env, amount: i32, to: Address);
    // fn vote(env : Env1)

    //create proposal and return its id
    fn c_prop(env: Env, proposal: Proposal) -> u32;

    //try to execute prop
    fn execute(env: Env, prop_id: u32);

    //get number of reputation or shares of the address
    fn shares(env: Env, of: Address) -> i32;

    //get total number of rep/shares that members hold
    fn tot_shares(env: Env) -> i32;

    //allow a member to vote on a proposal]
    fn vote(env: Env, prop_id: u32);
}


// Define the 'DaoContract' struct, which will implement the 'DaoTrait' contract functions
pub struct DaoContract;








































// fn init
// fn x_shares
// fn c_prop
// fn execute
// fn shares
// tot_shares
// vote






// execution

fn set_executed(env: &Env, prop_id: u32){
    env.data().set(DataKey::Executed(prop_id), true)
}

fn get_executed(env: &Env, prop_id: u32)-> bool{
    env.data().get(DataKey::Executed(prop_id))
    .unwrap_or(Ok(false))
    .unwrap()
}


// for voting check


fn voted(env: &Env, voter: Address, prop_id: u32) -> bool{
    return env.data().get(DataKey::Voted(ProposalVote {
        voter: voter.clone(),
        prop_id,
    })).unwrap_or(Ok(false)).unwrap()
}

// get the proposal id 


fn get_and_include_proposal_id(env: &Env) -> u32 {
    let prev = env
        .data()
        .get(DataKey::ProposalId)
        .unwrap_or(Ok(0u32))
        .unwrap();

    env.data().set(DataKey::ProposalId, prev + 1);
    prev
}

// get the bootstrap time

fn check_boot_strap(env: &Env) -> bool {
    env.data()
        .get::<_, u64>(DataKey::Bootstrap)
        .unwrap()
        .unwrap()
        > env.ledger().timestamp()
}



// check shares available

fn get_shares(env: &Env, of: Address) -> i32 {
    env.data()
        .get(DataKey::Balance(of))
        .unwrap_or(Ok(0))
        .unwrap()
}


// add the shares

fn add_shares(env: &Env, amount: i32, to: Address) {
    let current_shares = env
        .data()
        .get(DataKey::Balance(to.clone()))
        .unwrap_or(Ok(0))
        .unwrap();

    env.data()
        .set(DataKey::Balance(to), amount + current_shares);

    update_total_supply(env, amount)
}


fn update_total_supply(env: &Env, amount: i32) {
    let total_shares = tot_shares(env);

    env.data().set(DataKey::TotSupply, total_shares + amount)
}


fn total_shares(env: &Env) -> i32 {
    let total_share = env.data().get(DataKey::TotalSupply).unwrap_or(Ok(0))

    total_share
}





// admin functions
fn get_admin(env: &Env) -> Option<Result<Address, ConversionError>> {
    env.data().get(DataKey::Admin)
}

fn check_admin(env: &Env) -> bool {
    env.invoker() === env.data().get(DataKey::Admin).unwrap().unwrap()
}

fn set_admin(env: &Env, admin: Address) {
    env.data().set(DataKey::Admin, admin)
}


#[cfg(test)]
mod test