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
fn sub_operacao() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CalculadoraContract);
    let client = CalculadoraContractClient::new(&env, &contract_id);

    let x: u32 = 10;
    let y: u32 = 5;
    let result = client.sub(&x, &y);
    assert_eq!(result, 5);
}

#[test]
fn mul_operacao() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CalculadoraContract);
    let client = CalculadoraContractClient::new(&env, &contract_id);

    let x: u32 = 10;
    let y: u32 = 5;
    let result = client.mul(&x, &y);
    assert_eq!(result, 50);
}

#[test]
fn div_operacao() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CalculadoraContract);
    let client = CalculadoraContractClient::new(&env, &contract_id);

    let x: u32 = 10;
    let y: u32 = 5;
    let result = client.div(&x, &y);
    assert_eq!(result, 2);
}

#[test]
fn get_all_operations() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CalculadoraContract);
    let client = CalculadoraContractClient::new(&env, &contract_id);

    client.sum(&10, &20);
    client.sub(&5, &2);
    client.mul(&3, &3);
    client.div(&50, &2);

    let ops: Vec<Operation> = client.get_all_operacoes();
    assert_eq!(ops.get(0).unwrap().result, 30);
    assert_eq!(ops.get(1).unwrap().result, 3);
    assert_eq!(ops.get(2).unwrap().result, 9);
    assert_eq!(ops.get(3).unwrap().result, 25);
    assert_eq!(ops.len(), 4);

}