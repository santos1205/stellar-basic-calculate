#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn add_operacao() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CalculadoraContract);
    let client = CalculadoraContractClient::new(&env, &contract_id);

    let x: u32 = 10;
    let y: u32 = 5;
    let result = client.sum(&x, &y);
    assert_eq!(result, 15);
}

#[test]
fn get_all_operations() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CalculadoraContract);
    let client = CalculadoraContractClient::new(&env, &contract_id);

    client.sum(&10, &20);
    client.sum(&1, &2);
    client.sum(&2, &3);

    let ops: Vec<Operation> = client.get_all_operacoes();
    assert_eq!(ops.len(), 3);

    // MY-TODO: VAMOS PEGAR ESSA LISTA E CONFERIR TODAS AS D+ OPERAÇÕES.
}