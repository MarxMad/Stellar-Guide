#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, Client as TokenClient},
    Address, Env,
};

/// Crea el entorno, el admin, el token y registra el contrato de nómina.
fn setup() -> (Env, Address, Address, PayrollContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);

    // Registrar un token Stellar Asset nativo para testing
    let token_addr = env.register_stellar_asset_contract_v2(token_admin.clone());
    let sac = StellarAssetClient::new(&env, &token_addr.address());

    // Mintear tokens al admin para que pueda pagar nómina
    sac.mint(&admin, &10_000_000);

    // Registrar el contrato de nómina
    let contract_id = env.register(PayrollContract, ());
    let client = PayrollContractClient::new(&env, &contract_id);

    // Inicializar
    client.initialize(&admin, &token_addr.address());

    (env, admin, token_addr.address(), client)
}

#[test]
fn test_initialize() {
    let (env, admin, token_addr, client) = setup();

    // Verificar que get_all_employees retorna lista vacía
    let employees = client.get_all_employees();
    assert_eq!(employees.len(), 0);
}

#[test]
fn test_add_and_get_employee() {
    let (env, admin, _, client) = setup();

    let emp1 = Address::generate(&env);
    let salary: i128 = 5_000;

    client.add_employee(&emp1, &salary);

    let data = client.get_employee(&emp1);
    assert_eq!(data.address, emp1);
    assert_eq!(data.salary, 5_000);

    let all = client.get_all_employees();
    assert_eq!(all.len(), 1);
}

#[test]
fn test_add_multiple_employees() {
    let (env, _, _, client) = setup();

    let emp1 = Address::generate(&env);
    let emp2 = Address::generate(&env);
    let emp3 = Address::generate(&env);

    client.add_employee(&emp1, &1_000);
    client.add_employee(&emp2, &2_000);
    client.add_employee(&emp3, &3_000);

    let all = client.get_all_employees();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_remove_employee() {
    let (env, _, _, client) = setup();

    let emp1 = Address::generate(&env);
    let emp2 = Address::generate(&env);

    client.add_employee(&emp1, &1_000);
    client.add_employee(&emp2, &2_000);

    client.remove_employee(&emp1);

    let all = client.get_all_employees();
    assert_eq!(all.len(), 1);

    // Verificar que emp1 ya no existe
    let result = client.try_get_employee(&emp1);
    assert!(result.is_err());
}

#[test]
fn test_update_salary() {
    let (env, _, _, client) = setup();

    let emp1 = Address::generate(&env);
    client.add_employee(&emp1, &1_000);

    client.update_salary(&emp1, &2_500);

    let data = client.get_employee(&emp1);
    assert_eq!(data.salary, 2_500);
}

#[test]
fn test_disperse() {
    let (env, admin, token_addr, client) = setup();

    let emp1 = Address::generate(&env);
    let emp2 = Address::generate(&env);
    let emp3 = Address::generate(&env);

    client.add_employee(&emp1, &1_000);
    client.add_employee(&emp2, &2_000);
    client.add_employee(&emp3, &3_000);

    let total = client.disperse();
    assert_eq!(total, 6_000);

    // Verificar balances de empleados
    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&emp1), 1_000);
    assert_eq!(token.balance(&emp2), 2_000);
    assert_eq!(token.balance(&emp3), 3_000);

    // Verificar que el admin gastó 6_000
    assert_eq!(token.balance(&admin), 10_000_000 - 6_000);
}

#[test]
fn test_disperse_multiple_rounds() {
    let (env, admin, token_addr, client) = setup();

    let emp1 = Address::generate(&env);
    client.add_employee(&emp1, &500);

    // Primera dispersión
    let total1 = client.disperse();
    assert_eq!(total1, 500);

    // Segunda dispersión
    let total2 = client.disperse();
    assert_eq!(total2, 500);

    // Empleado recibió 2 pagos
    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&emp1), 1_000);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_double_initialize() {
    let (env, admin, token_addr, client) = setup();

    // Intentar inicializar de nuevo debe fallar
    client.initialize(&admin, &token_addr);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_add_duplicate_employee() {
    let (env, _, _, client) = setup();

    let emp1 = Address::generate(&env);
    client.add_employee(&emp1, &1_000);
    client.add_employee(&emp1, &2_000); // debe fallar
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_add_employee_zero_salary() {
    let (env, _, _, client) = setup();

    let emp1 = Address::generate(&env);
    client.add_employee(&emp1, &0); // debe fallar
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_disperse_no_employees() {
    let (_, _, _, client) = setup();

    // Dispersar sin empleados debe fallar
    client.disperse();
}
