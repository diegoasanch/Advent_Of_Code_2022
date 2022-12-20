use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Unable to read file");

    let mut seq = CharacterSequence::new(4);
    let mut packet_marker: Option<usize> = None;

    for (i, character) in input.chars().enumerate() {
        seq.add(character);

        if !seq.is_filled() {
            continue;
        }

        if seq.values_unique() {
            packet_marker = Some(i);
            break;
        }
    }

    if let Some(index) = packet_marker {
        println!("Packet starts at index {}", index + 1);
    } else {
        println!("String has no packet start marker");
    }
}

#[derive(Debug)]
struct CharacterSequence {
    values: Vec<char>,
    size: usize,
    pointer: usize,
}

impl CharacterSequence {
    /// Instantiates a CharacterSequence with the received size
    pub fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
            size,
            pointer: 0,
        }
    }

    /// Adds a value to the sequence, if sequence is at maximum capacity
    /// the oldest item is replaced
    pub fn add(&mut self, value: char) {
        if self.values.len() < self.size {
            self.values.push(value);
        } else {
            self.values[self.pointer] = value;
        }

        if self.pointer < self.size - 1 {
            self.pointer += 1;
        } else {
            self.pointer = 0;
        }
    }

    /// Determines if the values in the sequence are all unique
    pub fn values_unique(&self) -> bool {
        let mut uniq = HashSet::new();
        self.values.iter().all(|x| uniq.insert(x))
    }

    /// Determines if the sequence has its whole size occupied
    pub fn is_filled(&self) -> bool {
        self.values.len() == self.size
    }
}
