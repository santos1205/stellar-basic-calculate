#![no_std]
//use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
use soroban_sdk::{contract, contractimpl, contracttype, map, symbol_short, Env, Map, Symbol, Vec};

const OPERACOES: Symbol = symbol_short!("OPERACOES");
const TTL_THRESHOLD: u32 = 1000;
const TTL_EXTEND_TO: u32 = 5000;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OP {
    Sum,
    Sub,
    Mul,
    Div,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Operation {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub result: u32,
    pub tipo_op: OP,    
}

#[contract]
pub struct CalculadoraContract;

#[contractimpl]
impl CalculadoraContract {
    // ### FUNÇÕES DOS REGISTROS DAS OPERAÇÕES    
    pub fn get_all_operacoes(env: Env) -> Vec<Operation> {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        env.storage()
            .instance()
            .get::<Symbol, Map<u32, Operation>>(&OPERACOES)
            .unwrap_or(map![&env,])
            .values()
    }

    // ### FUNÇÕES DAS OPERAÇÕES BÁSICAS    
    // Função de soma
    pub fn sum(env: Env, x: u32, y: u32) -> u32 {
        let result: u32;
        result = x + y;

        let nova_op = Operation {
            id: 0,
            x,
            y,
            result,
            tipo_op: OP::Sum
        };
        
        // Salvando a operação
        let mut ops: Map<u32, Operation> = env.storage().instance().get(&OPERACOES).unwrap_or(map![&env,]);
        let novo_id = ops.len() as u32 + 1;
        ops.set(novo_id, nova_op);
        env.storage().instance().set(&OPERACOES, &ops);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        result
    }    

    // Função de subtração
    pub fn sub(env: Env, x: u32, y: u32) -> u32 {
        let result: u32;

        if x > y {
            result = x - y
        } else {
            result = y - x
        }

        let nova_op = Operation {
            id: 0,
            x,
            y,
            result,
            tipo_op: OP::Sub
        };

        // Salvando a operação
        let mut ops: Map<u32, Operation> = env.storage().instance().get(&OPERACOES).unwrap_or(map![&env,]);
        let novo_id = ops.len() as u32 + 1;
        ops.set(novo_id, nova_op);
        env.storage().instance().set(&OPERACOES, &ops);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        result
    }

    // Função de multiplicação
    pub fn mul(env: Env, x: u32, y: u32) -> u32 {
        let result: u32 = x * y;

        let nova_op = Operation {
            id: 0,
            x,
            y,
            result,
            tipo_op: OP::Mul
        };

        // Salvando a operação
        let mut ops: Map<u32, Operation> = env.storage().instance().get(&OPERACOES).unwrap_or(map![&env,]);
        let novo_id = ops.len() as u32 + 1;
        ops.set(novo_id, nova_op);
        env.storage().instance().set(&OPERACOES, &ops);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        result
    }

    // Função de divisão
    pub fn div(env: Env, x: u32, y: u32) -> u32 {
        let result: u32;
        if y == 0 {
            result = u32::MAX;
        } else {
            result = x / y;
        }

        let nova_op = Operation {
            id: 0,
            x,
            y,
            result,
            tipo_op: OP::Div
        };

        // Salvando a operação
        let mut ops: Map<u32, Operation> = env.storage().instance().get(&OPERACOES).unwrap_or(map![&env,]);
        let novo_id = ops.len() as u32 + 1;
        ops.set(novo_id, nova_op);
        env.storage().instance().set(&OPERACOES, &ops);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        result
    }
    
}

mod test;