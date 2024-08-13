/*
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically

company: HashMap = {
    "Dept1": Vec("A", "B"),
    "Dept2": Vec("C", "D", "E")
}
*/

use std::{
    collections::HashMap,
    io::{self, Write},
};

const HELP_MESSAGE: &str = r#"

Available commands:
    - 'Add <Name> to <Department>' to do exactly that
    - 'List <department>` to list every employee in the company in a tree-like structure
    - 'List all' to list every employee within this department
    - 'Exit' to stop AdminCLI
"#;

fn main() {
    println!("Welcome to AdminCLI. Do administrative things for a totally real company!");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();

    loop {
        println!("{}", HELP_MESSAGE);
        print!("Enter command: ");
        input.clear();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("\nerror: unable to read your input");
        let words: Vec<&str> = input.trim().split(' ').collect();
        match words.as_slice() {
            ["Add", name, "to", dept] => {
                company
                    .entry(dept.to_string())
                    .or_default()
                    .push(name.to_string());
                if let Some(x) = company.get_mut(dept.to_owned()) {
                    x[..].sort_unstable();
                }
            }
            ["List", "all"] => {
                for (dept, names) in &company {
                    println!("\n[{}]", dept);
                    for name in names {
                        println!("    {}", name);
                    }
                }
            }
            ["List", dept] => match company.get(*dept) {
                Some(names) => {
                    println!("\n[{}]", dept);
                    for name in names {
                        println!("    {}", name);
                    }
                }
                None => {
                    println!("\n'{}' department not found", dept);
                    continue;
                }
            },
            ["Exit"] => {
                println!("\nAdminCLI stopped ... Have a nice day\n");
                break;
            }
            _ => println!("\nunknown command, use only the defined commands"),
        }
    }
}
