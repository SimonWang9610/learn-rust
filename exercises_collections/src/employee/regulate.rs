use std::collections::HashMap;
use std::io;

pub fn add_employee(employees: &mut HashMap<String, Vec<String>>) {
    let command: String = read_input();
    if command == String::from("q") {
        ()
    } else {
        let command: Vec<&str> = command.split(' ').collect();

        if command.len() == 4 && command[2] == "to" {
            // why the ownership of command was not moved?
            let name: String = command[1].to_string();
            let department: String = command[3].to_string();
            add_info(name, department, employees);
        } else {
            println!("!!!!!Command is invalid");
            ()
        }
    }
}

pub fn delete_employee(employees: &mut HashMap<String, Vec<String>>) {
    let command: String = read_input();

    if command == String::from("q") {
        ()
    }
    let command: Vec<&str> = command.split(' ').collect();

    if command.len() == 4 && command[2] == "from" {
        let name: String = command[1].to_string();
        let department: String = command[3].trim().to_string();

        delete_info(name, department, employees);
    } else {
        println!("!!!!!Command is invalid");
        ()
    }
}

fn read_input() -> String {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read lines");
        input = input.trim().to_string();
        if input == String::from("q") {
            break input;
        }
        if input.starts_with("add") || input.starts_with("delete") {
            break input;
        } else {
            println!("!!!!!Follow ADD command: add NAME to DEPARTMENT");
            println!("!!!!!Follow DELETE command: delete NAME from DEPARTMENT\n");
            continue;
        }
    }
}

fn add_info(name: String, department: String, employees: &mut HashMap<String, Vec<String>>) -> () {
    if let Some(staff) = employees.get_mut(&department) {
        for person in staff.iter() {
            if *person == name {
                println!("\n@{} has been in #{}# department\n", name, department);
                ()
            }
        }

        staff.push(name);
    } else {
        let staff: Vec<String> = vec![name];
        employees.insert(department, staff);
    }
    ()
}

fn delete_info(
    name: String,
    department: String,
    employees: &mut HashMap<String, Vec<String>>,
) -> () {
    if let Some(staff) = employees.get_mut(&department) {
        let mut i: usize = 0;
        for (index, person) in staff.iter().enumerate() {
            if *person == name {
                i = index;
                break;
            }
        }
        staff.remove(i);
    } else {
        println!("\n@{} not in #{}# department\n", name, department);
    }
}
