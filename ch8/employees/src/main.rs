use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!(">>>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(_)  = io::stdin().read_line(&mut input) {
            println!("Failed to read line from standard input.");
            continue;
        }

        match input.trim() {
            "quit" => break,
            command_line => {
                let mut iter = command_line.split_whitespace();
                let command = iter.next();

                match command {
                    Some("Add") | Some("add") => {
                        let emp_name = iter.next().unwrap();
                        let second_command = iter.next();
                        match second_command {
                            Some("to") => {
                                let department_name = iter.next().unwrap();
                                let department = employees.entry(department_name.to_string()).or_insert(Vec::<String>::new());
                                if !department.contains(&emp_name.to_string()) {
                                    department.push(emp_name.to_string());
                                }
                            },
                            Some(&_) => continue,
                            None => continue,
                        }
                    },
                    Some("Get") | Some("get") => {
                        let dep_name = iter.next();
                        match dep_name {
                            Some(department_name) => {
                                match employees.get_mut(department_name) {
                                    Some(department) => {
                                        println!("Employees in department {}: {:?}", department_name, department);
                                    },
                                    None => {
                                        println!("Department {} does not exist.", department_name);
                                    }
                                }
                            },
                            None => {
                                println!("All employees: {:?}", employees);
                            }
                        }
                    },
                    Some(&_) => continue,
                    None => continue,
                }
                // println!("{:?}", employees);
            }
        }

    }
    println!("Done!");
}
