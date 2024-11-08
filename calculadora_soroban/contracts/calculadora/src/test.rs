#![cfg(test)]

use super::*;
use soroban_sdk::{Env};

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