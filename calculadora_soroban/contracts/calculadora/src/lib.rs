#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec};

// Estruturas do Enum e da Operação
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TipoOperacao {
    Sum,
    Sub,
    Multi,
    Div,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Operacao {
    pub x: u32,
    pub y: u32,
    pub result: u32,
    pub op: TipoOperacao,
    pub id_op: u32,
}

impl Operacao {
    pub fn new(x: u32, y: u32, result: u32, op: TipoOperacao, id_op: u32) -> Self {
        Operacao {
            x,
            y,
            result,
            op,
            id_op,
        }
    }
}

#[contract]
pub struct CalculadoraContract;

#[contractimpl]
impl CalculadoraContract {
    // ### Funções dos registros das operações
    // Função que incrementa o id da operação
    

    // Função para adicionar uma operação ao histórico
    

    // Função para retornar todas as operações
    

    // ### Funções das operações
    // Função de soma
    pub fn sum(env: Env, x: u32, y: u32) -> u32 {
        let result = x + y;
        // Obtenha o próximo ID auto-incrementado       
        result
    }

    // Função de subtração
    pub fn sub(env: Env, x: u32, y: u32) -> u32 {
        let result: u32;
        if x > y {
            result = x - y;
        } else {
            result = y - x;
        }
       
        result
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
