enum EmployeeType {
    Employee,
    Manager,
}

struct Employee {
    name: String,
    employee_type: EmployeeType,
    employees: Vec<Employee>,
}

impl Employee {
    fn new_employee(name: String) -> Self {
        Self {
            name,
            employee_type: EmployeeType::Employee,
            employees: Vec::new(),
        }
    }

    fn new_manager(name: String) -> Self {
        Self {
            name,
            employee_type: EmployeeType::Manager,
            employees: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_employee_type(&self) -> &EmployeeType {
        &self.employee_type
    }

    fn add_employee(&mut self, employee: Employee) -> Result<(), String> {
        match self.employee_type {
            EmployeeType::Manager => {
                self.employees.push(employee);
                Ok(())
            }
            _ => {
                println!("Only managers can add employees");
                Err("Only managers can add employees".into())
            }
        }
    }

    fn get_employees(&self) -> Result<&Vec<Employee>, String> {
        match self.employee_type {
            EmployeeType::Manager => {
                Ok(&self.employees)
            }
            _ => {
                println!("Only managers can get employees");
                Err("Only managers can get employees".into())
            }
        }
    }

    fn get_team_size(&self) -> Result<u32, String> {
        match self.employee_type {
            EmployeeType::Manager => {
                Ok(self.employees.len() as u32)
            }
            _ => {
                println!("Only managers can get team size");
                Err("Only managers can get team size".into())
            }
        }
    }
}

fn main() {
    let alice = Employee::new_employee("Alice".to_string());
    let bob = Employee::new_employee("Bob".to_string());

    let mut manager = Employee::new_manager("Eve".to_string());

    manager.add_employee(alice).unwrap();
    manager.add_employee(bob).unwrap();

    println!("Manager: {}", manager.get_name());
    println!("Team size: {}", manager.get_team_size().unwrap());
    match manager.get_employees() {
        Ok(employees) => {
            for employee in employees {
                println!("Employee: {}", employee.get_name());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
