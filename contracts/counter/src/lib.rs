#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    token, Address, Env, Vec,
};

/// Datos de un empleado registrado en la nómina.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeData {
    pub address: Address,
    pub salary: i128,
}

/// Claves de almacenamiento persistente del contrato.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Employee(Address),
    EmployeeList,
}

/// Errores del contrato de nómina.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PayrollError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    EmployeeNotFound = 3,
    EmployeeAlreadyExists = 4,
    InvalidSalary = 5,
    NoEmployees = 6,
}

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {
    /// Inicializa el contrato con el admin y la dirección del token de pago.
    ///
    /// # Arguments
    /// * `admin` - Dirección del administrador de la nómina
    /// * `token` - Dirección del contrato del token (ej. USDC, XLM wrapeado)
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
    ) -> Result<(), PayrollError> {
        // Verificar que no esté ya inicializado
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(PayrollError::AlreadyInitialized);
        }

        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Token, &token);

        // Inicializar lista vacía de empleados
        let employees: Vec<Address> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::EmployeeList, &employees);

        Ok(())
    }

    /// Agrega un empleado a la nómina.
    ///
    /// # Arguments
    /// * `employee` - Dirección del empleado
    /// * `salary` - Salario por dispersión (en unidades del token)
    pub fn add_employee(
        env: Env,
        employee: Address,
        salary: i128,
    ) -> Result<(), PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        if salary <= 0 {
            return Err(PayrollError::InvalidSalary);
        }

        // Verificar que no exista ya
        if env.storage().persistent().has(&DataKey::Employee(employee.clone())) {
            return Err(PayrollError::EmployeeAlreadyExists);
        }

        // Guardar datos del empleado
        let data = EmployeeData {
            address: employee.clone(),
            salary,
        };
        env.storage().persistent().set(&DataKey::Employee(employee.clone()), &data);

        // Agregar a la lista
        let mut list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(&env));
        list.push_back(employee);
        env.storage().persistent().set(&DataKey::EmployeeList, &list);

        Ok(())
    }

    /// Elimina un empleado de la nómina.
    pub fn remove_employee(
        env: Env,
        employee: Address,
    ) -> Result<(), PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        if !env.storage().persistent().has(&DataKey::Employee(employee.clone())) {
            return Err(PayrollError::EmployeeNotFound);
        }

        // Remover datos
        env.storage().persistent().remove(&DataKey::Employee(employee.clone()));

        // Remover de la lista
        let list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(&env));

        let mut new_list: Vec<Address> = Vec::new(&env);
        for addr in list.iter() {
            if addr != employee {
                new_list.push_back(addr);
            }
        }
        env.storage().persistent().set(&DataKey::EmployeeList, &new_list);

        Ok(())
    }

    /// Actualiza el salario de un empleado existente.
    pub fn update_salary(
        env: Env,
        employee: Address,
        new_salary: i128,
    ) -> Result<(), PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        if new_salary <= 0 {
            return Err(PayrollError::InvalidSalary);
        }

        let mut data: EmployeeData = env
            .storage()
            .persistent()
            .get(&DataKey::Employee(employee.clone()))
            .ok_or(PayrollError::EmployeeNotFound)?;

        data.salary = new_salary;
        env.storage().persistent().set(&DataKey::Employee(employee), &data);

        Ok(())
    }

    /// Consulta los datos de un empleado.
    pub fn get_employee(
        env: Env,
        employee: Address,
    ) -> Result<EmployeeData, PayrollError> {
        env.storage()
            .persistent()
            .get(&DataKey::Employee(employee))
            .ok_or(PayrollError::EmployeeNotFound)
    }

    /// Devuelve la lista de direcciones de todos los empleados.
    pub fn get_all_employees(env: Env) -> Result<Vec<Address>, PayrollError> {
        env.storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .ok_or(PayrollError::NotInitialized)
    }

    /// Dispersa la nómina: transfiere el salario a cada empleado registrado.
    ///
    /// El admin debe haber aprobado (allowance) al contrato para gastar
    /// la cantidad total, o bien firma la transacción directamente.
    ///
    /// Retorna el monto total dispersado.
    pub fn disperse(env: Env) -> Result<i128, PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(PayrollError::NotInitialized)?;

        let employee_list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .ok_or(PayrollError::NotInitialized)?;

        if employee_list.is_empty() {
            return Err(PayrollError::NoEmployees);
        }

        let token_client = token::Client::new(&env, &token_addr);
        let mut total: i128 = 0;

        for emp_addr in employee_list.iter() {
            let data: EmployeeData = env
                .storage()
                .persistent()
                .get(&DataKey::Employee(emp_addr.clone()))
                .ok_or(PayrollError::EmployeeNotFound)?;

            // Transferir del admin al empleado
            token_client.transfer(&admin, &data.address, &data.salary);
            total += data.salary;
        }

        Ok(total)
    }

    // ── Helpers internos ──────────────────────────────────────────

    fn get_admin(env: &Env) -> Result<Address, PayrollError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(PayrollError::NotInitialized)
    }
}

mod test;
