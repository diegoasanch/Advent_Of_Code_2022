use std::fs;

use parser::parse_file_system;

mod directory;
mod file;
mod file_system;
mod inode;
mod parser;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Input file not found");

    let file_system = parse_file_system(&input);

    println!("{:#?}", file_system);
}
