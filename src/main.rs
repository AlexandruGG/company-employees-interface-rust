use itertools::Itertools;
use std::collections::HashMap;
use std::{io, process};

fn main() {
    println!("Add employees: 'Add Name to Department'");

    const KEYWORD_ADD: &str = "Add";
    const KEYWORD_TO: &str = "to";

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let user_input = read_input();

        if &user_input == "done" {
            break;
        } else {
            println!("\nAdd more? (type 'done' to quit)")
        }

        let (name, department) = read_command(user_input, KEYWORD_ADD, KEYWORD_TO);

        let dept_employees = company.entry(department).or_insert_with(|| vec![]);
        dept_employees.push(name);
    }

    println!("\nEnter department to retrieve employees (type 'done' to quit)");

    loop {
        let user_input = read_input();
        if &user_input == "done" {
            break;
        }

        match company.get(&user_input) {
            Some(list) => println!("{:?}", list),
            None => println!("Department does not exist!"),
        };

        println!("\nRetrieve another? (type 'done' to quit)")
    }
}

fn read_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input.truncate(user_input.trim().len());

    user_input
}

fn read_command(user_input: String, add: &str, to: &str) -> (String, String) {
    let mut errors: Vec<&str> = Vec::new();

    if !user_input.contains(add) {
        errors.push("Invalid Command: Missing <Add>");
    }

    if !user_input.contains(to) {
        errors.push("Invalid Command: Missing <to>");
    }

    let mut parts: Vec<_> = user_input.split(' ').collect();

    if parts.len() < 4 {
        errors.push("Invalid Command! Format: <Add Name to Department>");
    }

    if !errors.is_empty() {
        for error in errors {
            println!("{}", error);
        }

        process::exit(1);
    }

    parts.retain(|&p| !p.is_empty() && p != add && p != to);

    let (name, department) = parts.iter().next_tuple().unwrap();

    (name.to_string(), department.to_string())
}
