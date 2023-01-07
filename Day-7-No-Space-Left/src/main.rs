use std::fs;

use parser::parse_file_system;

use crate::inode::{INode, INodeRef};

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

    let size_limit = 100000u32;
    let all_items = file_system.all_items();

    let small_items = all_items
        .iter()
        .filter(|item| {
            let item = item.try_borrow().expect("Dead item reference");
            item.is_directory() && item.size() < size_limit
        })
        .collect::<Vec<&INodeRef>>();

    let total_size = small_items.iter().fold(0, |acc, item| {
        acc + item.try_borrow().expect("Dead item reference").size()
    });

    println!(
        "Total size of directories under {} bytes: {}",
        size_limit, total_size
    );
}
