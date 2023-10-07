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

#[test]

fn test() {
    let env = Env::default();

    let user_1 = env.accounts().generate();

    let (dao_contract, client) = create_dao_contract(&env, &user_1);
    let (token_contract_id, token_client) = create_token_contract(&env, &user_1);


    assert_eq!(
        1,
        client
            .with_source_account(&user_1)
            .shares(&Account(user_1.clone()))
    );

    env.ledger().set(LedgerInfo {
        timestamp: env.ledger().timestamp() + 1,
        protocol_version: 1,
        sequence_number: 1,
        network_passphrase: Default::default(),
        base_reserve: 1,
    });
    




}