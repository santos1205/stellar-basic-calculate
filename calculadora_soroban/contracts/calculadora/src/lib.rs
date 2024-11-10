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
    fn add_operacao(env: Env, op: Operation) -> u32 {
        let mut ops: Map<u32, Operation> = env.storage().instance().get(&OPERACOES).unwrap_or(map![&env,]);

        let novo_id = ops.len() as u32 + 1;
        let nova_op = Operation {
            id: op.id,
            x: op.x,
            y: op.y,
            tipo_op: op.tipo_op,
            result: op.result
        };

        ops.set(op.id, nova_op);
        env.storage().instance().set(&OPERACOES, &ops);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        novo_id    
    }

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
        
        //Self::add_operacao(_env, nova_op);
        let mut ops: Map<u32, Operation> = env.storage().instance().get(&OPERACOES).unwrap_or(map![&env,]);
        let novo_id = ops.len() as u32 + 1;        

        ops.set(novo_id, nova_op);
        env.storage().instance().set(&OPERACOES, &ops);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        result
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
    pub fn mul(_env: Env, x: u32, y: u32) -> u32 {
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