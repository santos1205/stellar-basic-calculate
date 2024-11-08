#![cfg(test)]

use super::*;
use soroban_sdk::{Env};

#[test]
fn test_sum() {
    let env = Env::default();
    let result = CalculadoraContract::sum(env, 2, 3);
    assert_eq!(result, 5);
}