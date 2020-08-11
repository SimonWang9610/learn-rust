// mod list;
// use crate::list::list as other_list;

// mod pig_latin;
// use crate::pig_latin::pig;

mod employee;

use crate::employee::display;
use crate::employee::regulate;

use std::collections::HashMap;
use std::io;

fn main() {
    /* let mut v = vec![10, 5, 6, 11, 9, 10, 27];
    let mean = other_list::mean(&v);
    let median = other_list::median(&mut v);
    let mode = other_list::mode(&v);
    println!("mean {} median {} mode {:?}", mean, median, mode);

    let s = "hello world apple orange pear";
    let pig = pig::pig_latin(s);
    println!("pig latin {}", pig); */
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!(
            "Commands:\n
            1 add new employee.\n
            2 display all employees.\n
            3 display employees in one department.\n
            4 delete one employee.\n
            0 quit\n"
        );

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read lines");

        let command: u8 = match command.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                panic!("Error: {}", error);
            }
        };

        match command {
            1 => {
                println!("-------Add new employee--------");
                println!("*ADD command: add NAME to DEPARTMENT");
                regulate::add_employee(&mut employees);
                continue;
            }
            2 => {
                println!("------Display all employees in this company---------");
                display::all_employees(&mut employees);
                continue;
            }
            3 => {
                println!("------Display employees in this department------");
                display::department_employees(&mut employees);
                continue;
            }
            4 => {
                println!("-----Delete employee-------");
                println!("*DELETE command: delete NAME from DEPARTMENT");
                regulate::delete_employee(&mut employees);
                continue;
            }
            0 => break,
            _ => {
                println!("*******Please enter valid command!**********");
                continue;
            }
        }
    }
}
