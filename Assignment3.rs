use std::io;

struct Employee {
    employee_name: String,
    employee_id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

impl Employee {
    // Constructor
    fn new(
        name: String,
        id: i32,
        email: String,
        age: i32,
        phone_number: String,
    ) -> Employee {
        Employee {
            employee_name: name,
            employee_id: id,
            email,
            age,
            phone_number,
        }
    }
}

fn get_employee_by_id(employees: &Vec<Employee>, target_id: i32) -> Option<&Employee> {
    for emp in employees {
        if emp.employee_id == target_id {
            return Some(emp);
        }
    }
    None
}

fn get_employees_by_age(employees: &Vec<Employee>, target_age: i32) -> Vec<&Employee> {
    employees
        .iter()
        .filter(|&emp| emp.age == target_age)
        .collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("Enter employee_name (or type 'exit' to stop):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let employee_name = input.trim().to_string();

        if employee_name == "exit" {
            break;
        }

        println!("Enter employee_id:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let employee_id: i32 = input.trim().parse().expect("Invalid input");

        println!("Enter email:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");
        let email = email.trim().to_string();

        println!("Enter age:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let age: i32 = input.trim().parse().expect("Invalid input");

        println!("Enter phone_number:");
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");
        let phone_number = phone_number.trim().to_string();

        let new_employee = Employee::new(
            employee_name,
            employee_id,
            email,
            age,
            phone_number,
        );
        employees.push(new_employee);
    }

    // Get employee details by ID
    println!("Enter employee_id to search:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let search_id: i32 = input.trim().parse().expect("Invalid input");

    if let Some(found_employee) = get_employee_by_id(&employees, search_id) {
        println!("Employee found:");
        println!("Name: {}", found_employee.employee_name);
        println!("ID: {}", found_employee.employee_id);
        println!("Email: {}", found_employee.email);
        println!("Age: {}", found_employee.age);
        println!("Phone: {}", found_employee.phone_number);
    } else {
        println!("Employee with ID {} not found.", search_id);
    }

    // Get employees with the same age
    println!("Enter age to search for employees with the same age:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let search_age: i32 = input.trim().parse().expect("Invalid input");

    let same_age_employees = get_employees_by_age(&employees, search_age);
    if !same_age_employees.is_empty() {
        println!("Employees with age {}:", search_age);
        for emp in same_age_employees {
            println!("Name: {}, ID: {}", emp.employee_name, emp.employee_id);
        }
    } else {
        println!("No employees found with age {}.", search_age);
    }
}