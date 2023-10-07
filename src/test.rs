#[cfg(test)]
use super::DaoContract;
use super::DaoContractClient;
use crate::{
    token::{self, Identifier, TokenMetadata, Signature},
    Address::Account,
    Proposal, ProposalInstr,
};
use soroban_sdk::{
    symbol,
    testutils::{Accounts, Ledger, LedgerInfo},
    vec, AccountId, Address, BytesN, Env, IntoVal, BigInt,
};



fn create_token_contract(e: &Env, admin: &AccountId) -> (BytesN<32>, token::Client) {
    let id = e.register_contract_token(None);
    let token = token::Client::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: "name".into_val(e),
            symbol: "symbol".into_val(e),
            decimals: 7,
        },
    );
    (id.into(), token)
}

fn create_dao_contract(e: &Env, admin: &AccountId) -> (BytesN<32>, DaoContractClient) {
    let contract_id = e.register_contract(None, DaoContract);
    let client = DaoContractClient::new(&e, &contract_id);
    client.with_source_account(admin).init();
    (contract_id.into(), client)
}
