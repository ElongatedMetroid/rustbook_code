use std::collections::HashMap;
use std::io;

struct Company {
    // The key will we the department and the vector of strings will be the people in the department
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company { departments: HashMap::new() }
    }

    fn add(&mut self, dep: &str, name: &str) {
        self
            .departments
            .entry(dep.to_string())
            .or_insert(Vec::new())
            .push(name.to_string());
    }

    fn list_dep(&self, dep: &str) {
        match self.departments.get(&dep.to_string()) {
            Some(_) => (),
            None => {
                println!("Invalid Department!");
                return;
            }
        }

        println!("{} -", dep);
        // TODO 
        for name in self.departments.get(&dep.to_string()).expect("Invalid Department").iter() {
            println!("\t{}", name);
        }
    }

    fn list_company(&self) {
        for (dep, names) in self.departments.iter() {
            println!("{} -", dep);
            for name in names.iter() {
                println!("\t{}", name);
            }
        }
    }
}

fn main() {
    let mut company = Company::new();
    // "Add Sally to Engineering" "Add Amir to Sales"
    
    loop {
        println!("\nEnter a command (type HELP for help)");

        let mut input = String::new();
        
        io::stdin().read_line(&mut input)
            .expect("Failed to read line from stdin");

        match input.as_str().split_whitespace().nth(0) {
            Some(s) => match s {
                "HELP" => {
                    println!("-- GUIDE --");
                    println!("ADD: ADD name department");
                    println!("LIST: LIST department (Use ALL to list all departments");
                    println!("EXIT: EXIT");
                },

                "ADD" => {
                    let name = match input.as_str().split_whitespace().nth(1) {
                        Some(n) => n,
                        None => {
                            println!("Please provide a name with the ADD command");
                            continue;
                        },
                    };

                    let dep = match input.as_str().split_whitespace().nth(2) {
                        Some(n) => n,
                        None => {
                            println!("Please provide a department with the ADD command");
                            continue;
                        },
                    };

                    company.add(dep, name);
                    println!("Added {} to {}", name, dep);
                }

                "LIST" => {
                    let dep = match input.as_str().split_whitespace().nth(1) {
                        Some(n) => n,
                        None => {
                            println!("Please provide a department (or ALL) with the LIST command");
                            continue;
                        },
                    };

                    if dep == "ALL" {
                        company.list_company();
                    } else {
                        company.list_dep(dep);
                    }
                }

                "EXIT" => return,
    
                _ => {
                    println!("{}: command not found", s);
                    continue;
                },
            },
            None => {
                println!("Input has zero length");
                continue;
            }
        }
    }
}
