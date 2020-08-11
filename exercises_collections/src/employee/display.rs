// use std::any::type_name;
use std::collections::HashMap;
use std::io;

pub fn all_employees(employees: &mut HashMap<String, Vec<String>>) {
    for (department, staff) in employees.iter_mut() {
        println!("#{}# DEPARTMENT", department);
        staff.sort();
        println!("{:?}", staff);
    }
}

pub fn department_employees(employees: &mut HashMap<String, Vec<String>>) {
    let department = read_input(employees);

    if department == String::from("q") {
        ()
    } else {
        println!("All employees in #{}# DEPARTMENT as below:", department);
        match employees.get_mut(&department) {
            Some(v) => {
                v.sort();
                println!("{:?}", v);
            }
            None => {
                println!("There is no employee in #{}# DEPARTMENT\n", department);
            }
        }
    }
}

fn read_input(employees: &HashMap<String, Vec<String>>) -> String {
    loop {
        let mut exist: bool = false;

        for department in employees.keys() {
            println!("#DEPARTMENT NAME#: {}", department);
        }

        println!("Enter the department:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read lines");
        // read_line() also read 'Enter` as \n\n
        // trim() can keep the input as expected
        input = input.trim().to_string();

        if input == String::from("q") {
            break input;
        }

        for department in employees.keys() {
            if &input == department {
                exist = true;
            }
        }

        if exist {
            break input;
        } else {
            println!("!!!!!You can only choose from the above list!\n");
            continue;
        }
    }
}

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }
