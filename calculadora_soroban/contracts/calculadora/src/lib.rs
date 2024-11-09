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
    fn get_next_id(env: &Env) -> u32 {
        let current_id: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(env, "operation_id"))
            .unwrap_or(0); // Se não encontrar, começa com 0
        let next_id = current_id + 1;
        env.storage()
            .instance()
            .set(&Symbol::new(env, "operation_id"), &next_id);

        next_id
    }

    // Função para adicionar uma operação ao histórico
    fn add_operation(env: &Env, op: Operacao) {
        let key = Symbol::new(env, "operacoes");

        // Carrega o histórico de operações ou inicia um novo vetor se estiver vazio
        let mut operacoes: Vec<Operacao> = env
            .storage()
            .instance()
            .get::<Symbol, Vec<Operacao>>(&Symbol::new(env, "operacoes"))
            .unwrap_or(Vec::new(env));
        operacoes.push_back(op);

        // Atualiza o histórico no armazenamento
        env.storage().instance().set(&key, &operacoes);
    }

    // Função para retornar todas as operações
    pub fn all_op(env: Env) -> Vec<Operacao> {        
        env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "operacoes"))
            .unwrap_or(Vec::new(&env))
    }

    // ### Funções das operações
    // Função de soma
    pub fn sum(env: &Env, x: u32, y: u32) -> u32 {
        let result = x + y;
        // Obtenha o próximo ID auto-incrementado
        let id = get_next_id(env);
        let operacao = Operacao::new(x, y, result, TipoOperacao::Sum, id);
        Self::add_operation(&env, operacao);
        result
    }

    // Função de subtração
    pub fn sub(env: &Env, x: u32, y: u32) -> u32 {
        let result: u32;
        if x > y {
            result = x - y;
        } else {
            result = y - x;
        }
        let id = get_next_id(env);
        let operacao = Operacao::new(x, y, result, TipoOperacao::Sub, id);
        Self::add_operation(env, operacao);

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
