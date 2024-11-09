#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_sum() {
    let env = Env::default();
    let result = CalculadoraContract::sum(env, 2, 3);
    assert_eq!(result, 5);
}

#[test]
fn test_sub() {
    let env: Env = Env::default();
    let result = CalculadoraContract::sub(env, 10, 4);
    assert_eq!(result, 6);
}

#[test]
fn test_mult() {
    let env: Env = Env::default();
    let result = CalculadoraContract::mult(env, 5, 5);
    assert_eq!(result, 25);
}

#[test]
fn test_div() {
    let env = Env::default();
    let result = CalculadoraContract::div(env, 10, 2);
    assert_eq!(result, 5);
}

#[test]
fn test_div_por_zero() {
    let env = Env::default();
    let result = CalculadoraContract::div(env, 8, 0);
    assert_eq!(result, u32::MAX);
}

#[test]
fn test_operations_listing() {
    let env = Env::default();
    let calc = CalculadoraContract::new(&env);

    // Realizar algumas operações
    calc.sum(&env, 5, 3);
    calc.sub(&env, 10, 4);
    calc.mul(&env, 6, 7);
    calc.div(&env, 15, 5);

    // Testa listagem de todas as operações
    let all_ops = calc.all_op(&env);
    assert_eq!(all_ops.len(), 4); // Espera-se que haja 4 operações registradas

    // Verificação específica de operações
    assert_eq!(all_ops[0].x, 5);
    assert_eq!(all_ops[0].y, 3);
    assert_eq!(all_ops[0].result, 8); // 5 + 3
    assert_eq!(all_ops[1].result, 6); // 10 - 4
    assert_eq!(all_ops[2].result, 42); // 6 * 7
    assert_eq!(all_ops[3].result, 3); // 15 / 5
}
