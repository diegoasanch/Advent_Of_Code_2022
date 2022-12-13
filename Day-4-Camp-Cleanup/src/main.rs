use core::fmt;
use std::{fs, process::exit};

struct Range(i32, i32);

#[derive(Debug)]
enum RangeError {
    MissingBoundsSeparator,
    MissingPairSeparator,
    InvalidBound,
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingBoundsSeparator => "Missing bounds separator (Hyphen \"-\")",
            Self::MissingPairSeparator => "Missing pair separator (Comma \",\")",
            Self::InvalidBound => "Could not parse bound into number",
        };
        write!(f, "{message}")
    }
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn partially_contains(&self, other: &Range) -> bool {
        if self.0 <= other.0 {
            self.1 >= other.0
        } else {
            self.0 <= other.1
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("File is missing.");
    let mut full_overlap = 0;
    let mut partial_overlap = 0;

    for (i, line) in input.lines().enumerate() {
        match get_ranges(line) {
            Ok((range_1, range_2)) => {
                if range_1.fully_contains(&range_2) || range_2.fully_contains(&range_1) {
                    full_overlap += 1;
                }
                if range_1.partially_contains(&range_2) {
                    partial_overlap += 1;
                }
            }
            Err(error) => {
                eprintln!(
                    "Encountered an error while processing range \"{line}\" at line {}.",
                    i + 1
                );
                eprintln!("Error: {error}");
                eprintln!("exiting.");
                exit(1)
            }
        }
    }

    println!("Result");
    println!("------");
    println!("Full overlap: {full_overlap}");
    println!("Partial overlap: {partial_overlap}");
}

fn get_range(from: &str) -> Result<Range, RangeError> {
    let mut parts = from.split('-');
    let from = parts.next().ok_or(RangeError::MissingBoundsSeparator)?;
    let to = parts.next().ok_or(RangeError::MissingBoundsSeparator)?;

    Ok(Range(
        from.parse().or(Err(RangeError::InvalidBound))?,
        to.parse().or(Err(RangeError::InvalidBound))?,
    ))
}

fn get_ranges(from_line: &str) -> Result<(Range, Range), RangeError> {
    let mut parts = from_line.split(',');
    Ok((
        get_range(parts.next().ok_or(RangeError::MissingPairSeparator)?)?,
        get_range(parts.next().ok_or(RangeError::MissingPairSeparator)?)?,
    ))
}
