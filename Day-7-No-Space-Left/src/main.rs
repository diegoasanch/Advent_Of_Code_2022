use std::fs;

use parser::parse_file_system;

use crate::inode::INode;

mod directory;
mod file;
mod file_system;
mod inode;
mod parser;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Input file not found");

    let file_system = parse_file_system(&input).expect("Failed to parse input");

    println!("Tree");
    file_system.print_tree(0);
}
