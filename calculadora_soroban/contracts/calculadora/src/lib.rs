#![no_std]
//use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct CalculadoraContract;

#[contractimpl]
impl CalculadoraContract {
    // Função de soma
    pub fn sum(_env: Env, x: u32, y: u32) -> u32 {
        x + y
    }    
}

mod test;