use std::{fs, rc::Rc};

use parser::parse_file_system;

use crate::inode::{INode, INodeRef};

mod directory;
mod file;
mod file_system;
mod inode;
mod parser;

const DISK_SIZE: u32 = 70_000_000;
const SIZE_LIMIT: u32 = 100_000;
const REQUIRED_UPDATE_SIZE: u32 = 30_000_000;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Input file not found");

    let file_system = parse_file_system(&input).expect("Failed to parse input");

    println!("Tree");
    file_system.print_tree(0);

    let all_items = file_system.all_items();

    let small_items = all_items
        .iter()
        .filter(|item| {
            let item = item.try_borrow().expect("Dead item reference");
            item.is_directory() && item.size() < SIZE_LIMIT
        })
        .collect::<Vec<&INodeRef>>();

    let total_size = small_items.iter().fold(0, |acc, item| {
        acc + item.try_borrow().expect("Dead item reference").size()
    });

    println!(
        "Total size of directories under {} bytes: {}",
        SIZE_LIMIT, total_size
    );

    let total_size = file_system.size();
    let free_space = DISK_SIZE - total_size;

    let required_delete_size = REQUIRED_UPDATE_SIZE - free_space;

    let smallest_sufficient_directory =
        find_smallest_sufficient_directory(&all_items, required_delete_size);

    println!(
        "Smallest directory with sufficient space: {:?}",
        smallest_sufficient_directory
    );
}

fn find_smallest_sufficient_directory(
    items: &Vec<INodeRef>,
    required_space: u32,
) -> Option<INodeRef> {
    let mut smallest_sufficient_directory = None;
    let mut smallest_sufficient_directory_size = u32::MAX;

    for item in items {
        let borrowed_item = item.try_borrow().expect("Dead item reference");
        if borrowed_item.is_directory() && borrowed_item.size() >= required_space {
            if borrowed_item.size() < smallest_sufficient_directory_size {
                smallest_sufficient_directory = Some(Rc::clone(&item));
                smallest_sufficient_directory_size = borrowed_item.size();
            }
        }
    }

    smallest_sufficient_directory
}
