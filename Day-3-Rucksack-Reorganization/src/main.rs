use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");
    let mut result = 0;
    let mut groups = 0;
    let mut group = Vec::new();

    for line in input.lines() {
        let rucksack = Rucksack::from(line);
        let repeated = rucksack.find_repeated().expect("Did not found repeated");
        let value = get_char_priority(&repeated);
        result += value;

        if group.len() < 2 {
            group.push(rucksack);
        } else {
            group.push(rucksack);
            let badge = get_group_badge(
                group.pop().unwrap(),
                group.pop().unwrap(),
                group.pop().unwrap(),
            )
            .expect("Group does not share a badge");
            groups += get_char_priority(&badge);
        }
    }

    println!("Result: {} Groups: {}", result, groups);
}

/// Gets priority from char based on its ASCII value
/// # Panics
/// If char is not alphabetic
fn get_char_priority(c: &char) -> u32 {
    match c {
        'a'..='z' => *c as u32 - 96,
        'A'..='Z' => *c as u32 - 65 + 27,
        _ => panic!("Invalid item {}", c),
    }
}

/// Finds a common `char` between the received Rucksacks
fn get_group_badge(item1: Rucksack, item2: Rucksack, item3: Rucksack) -> Option<char> {
    item1.items.chars().find(|char1| {
        item2.items.chars().any(|char2| *char1 == char2)
            && item3.items.chars().any(|char3| *char1 == char3)
    })
}

#[derive(Debug)]
struct Rucksack {
    items: String,
    middle: usize,
}

impl Rucksack {
    pub fn from(input: &str) -> Self {
        let len = input.len();
        if len % 2 != 0 {
            panic!("Rucksack contents is not even quantity {}", len);
        }

        Self {
            items: input.to_owned(),
            middle: len / 2,
        }
    }

    pub fn compartment_1(&self) -> &str {
        &self.items[..self.middle]
    }

    pub fn compartment_2(&self) -> &str {
        &self.items[self.middle..]
    }

    pub fn find_repeated(&self) -> Option<char> {
        self.compartment_1()
            .chars()
            .find(|char1| self.compartment_2().chars().any(|char2| *char1 == char2))
    }
}
