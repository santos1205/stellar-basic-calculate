#![no_std]
//use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
use soroban_sdk::{contract, contractimpl, Env, Vec};

// Estruturas do Enum e da Operação
#[derive(Clone)]
pub enum TipoOperacao {
    Sum,
    Sub,
    Multi,
    Div
}

#[derive(Clone)]
pub struct Operacao {
    pub x: u32,
    pub y: u32,
    pub result: u32,
    pub op: TipoOperacao,
    pub id_op: u32
}

impl Operacao {
    pub fn new(x: u32, y: u32, result: u32, op: TipoOperacao, id_op: u32) -> Self {
        Operacao { x, y, result, op, id_op }
    }
}

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