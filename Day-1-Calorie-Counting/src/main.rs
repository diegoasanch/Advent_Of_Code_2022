use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");

    let max_count = 3;
    let mut max_carriers = Vec::with_capacity(max_count);

    let mut max_calories = 0;
    let mut current_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current_elf > max_calories {
                max_calories = current_elf;
            }
            push_if_top(&mut max_carriers, current_elf, max_count);
            current_elf = 0;
        } else {
            current_elf += line.trim().parse::<u32>().expect("Invalid number.")
        }
    }
    if current_elf > max_calories {
        max_calories = current_elf;
    }
    push_if_top(&mut max_carriers, current_elf, max_count);

    println!("{}", max_calories);

    println!("Top {} elves: {:#?}", max_count, max_carriers);
    println!("Sum of top elves: {}", max_carriers.iter().sum::<u32>());
}

/// Replaces the minimum value in the vector if `num` is greater than it
/// and the array has reached its capacity
fn push_if_top(arr: &mut Vec<u32>, num: u32, capacity: usize) {
    if arr.len() >= capacity {
        let min_index = find_min_index(&arr);
        if num > arr[min_index] {
            arr[min_index] = num;
        }
    } else {
        // Array is empty
        arr.push(num);
    }
}

/// Returns the index of the minimum value in the vector
/// If the minimum values is present more than once, the first index will be returned
/// Panics if vector is empty
fn find_min_index(arr: &Vec<u32>) -> usize {
    assert!(arr.len() > 0, "Array is empty");

    let mut min_index = 0;
    let mut min_val = arr[0];

    for (i, num) in arr.iter().enumerate() {
        if *num < min_val {
            min_val = *num;
            min_index = i;
        }
    }
    min_index
}
