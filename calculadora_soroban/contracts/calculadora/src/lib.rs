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

    // Função de subtração
    pub fn sub(_env: Env, x: u32, y: u32) -> u32 {
        if x > y {
            x - y
        } else {
            y - x
        }
    }

    // Função de multiplicação
    pub fn mult(_env: Env, x: u32, y: u32) -> u32 {
        x * y
    }

    // Função de divisão
    pub fn div(_env: Env, x: u32, y: u32) -> u32 {
        if y == 0 {
            u32::MAX
        } else {
            x / y
        }
    }
}

mod test;