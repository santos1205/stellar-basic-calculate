use soroban_sdk::{Env, testutils::{Ledger, Address}};
use crate::{CalculadoraContract, Operacao, TipoOperacao}; // Importar o contrato e as structs

#[test]
fn test_operations_and_list_all() {
    let env = Env::default();
    let calc = CalculadoraContract;

    // Realizando as 4 operações básicas
    CalculadoraContract::sum(env.clone(), 10, 5);  // Soma
    CalculadoraContract::sub(env.clone(), 10, 3);  // Subtração
    CalculadoraContract::mult(env.clone(), 4, 5);   // Multiplicação
    CalculadoraContract::div(env.clone(), 20, 4);  // Divisão

    // Listando todas as operações realizadas
    //let operations = calc.all_op(env.clone());

    // Verificando se o número total de operações realizadas é 4
    //assert_eq!(operations.len(), 4);

    // Verificando se as operações estão corretas
    // assert_eq!(operations[0].op, TipoOperacao::Sum);
    // assert_eq!(operations[0].x, 10);
    // assert_eq!(operations[0].y, 5);
    // assert_eq!(operations[0].result, 15);

    // assert_eq!(operations[1].op, TipoOperacao::Sub);
    // assert_eq!(operations[1].x, 10);
    // assert_eq!(operations[1].y, 3);
    // assert_eq!(operations[1].result, 7);

    // assert_eq!(operations[2].op, TipoOperacao::Mul);
    // assert_eq!(operations[2].x, 4);
    // assert_eq!(operations[2].y, 5);
    // assert_eq!(operations[2].result, 20);

    // assert_eq!(operations[3].op, TipoOperacao::Div);
    // assert_eq!(operations[3].x, 20);
    // assert_eq!(operations[3].y, 4);
    // assert_eq!(operations[3].result, 5);
}
