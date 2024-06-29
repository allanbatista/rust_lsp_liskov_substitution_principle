trait IEmployee {
    fn get_name(&self) -> String;
}

trait IManager: IEmployee {
    fn get_team_size(&self) -> u32;
    fn add_employee(&mut self, employee: Box<dyn IEmployee>);
    fn get_employees(&self) -> &Vec<Box<dyn IEmployee>>;
}

trait IManaged {
    fn add_manager(&mut self, employee: Box<dyn IManager>);
}

struct Employee {
    name: String
}

impl IEmployee for Employee {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct Manager {
    name: String,
    employees: Vec<Box<dyn IEmployee>>,
}

impl IEmployee for Manager {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl IManager for Manager {
    fn get_team_size(&self) -> u32 {
        self.employees.len() as u32
    }

    fn add_employee(&mut self, employee: Box<dyn IEmployee>) {
        self.employees.push(employee);
    }

    fn get_employees(&self) -> &Vec<Box<dyn IEmployee>> {
        &self.employees
    }
}

fn main() {
    let alice: Box<dyn IEmployee> = Box::new(Employee {
        name: "Alice".to_string()
    });

    let bob: Employee = Employee {
        name: "Bob".to_string()
    };

    let mut manager: Box<dyn IManager> = Box::new(Manager {
        name: "Eve".to_string(),
        employees: Vec::new(),
    });

    manager.add_employee(alice);
    manager.add_employee(Box::new(bob));

    println!("Manager: {}", manager.get_name());
    println!("Team size: {}", manager.get_team_size());
    for employee in manager.get_employees() {
        println!("Employee: {}", employee.get_name());
    }
}
