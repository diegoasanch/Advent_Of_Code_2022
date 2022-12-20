use std::{cell::RefCell, fs, process::exit};

use crate::stack::Stack;

mod instruction;
mod parser;
mod stack;

fn main() {
    let input_text = match fs::read_to_string("./src/input.txt") {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Could not read input file. Error: {e}");
            exit(1);
        }
    };

    let parsed_input = match parser::parse_input(&input_text) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Could not parse input. Error: {e}");
            exit(1);
        }
    };

    for instruction in parsed_input.instructions {
        if let (Some(from), Some(to)) = (
            parsed_input.crates.get(&instruction.from),
            parsed_input.crates.get(&instruction.to),
        ) {
            if let Err(err) = from
                .borrow_mut()
                .move_amount(&mut to.borrow_mut(), instruction.amount as usize)
            {
                eprintln!("Could not move amount. Error: {err}");
                exit(1);
            }
        }
    }

    let mut entries: Vec<(&String, &RefCell<Stack>)> = parsed_input.crates.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let mut output = String::new();
    for entry in entries {
        if let Some(top) = entry.1.borrow().peek() {
            output += &top;
        }
    }

    println!("Top items {:?}", output);
}
