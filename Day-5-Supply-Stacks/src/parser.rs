use std::{cell::RefCell, collections::HashMap, fmt};

use crate::{instruction::Instruction, stack::Stack};
use anyhow::Result;

#[derive(Debug)]
enum ParseError {
    InvalidLabels,
    InvalidCrateLabel(String),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::InvalidLabels => String::from("Input has invalid crate labels"),
            Self::InvalidCrateLabel(label) => {
                format!("Input has invalid crate label: \"{label}\"")
            }
        };
        write!(f, "{error}")
    }
}

#[derive(Debug)]
pub struct Input {
    pub instructions: Vec<Instruction>,
    pub crates: HashMap<String, RefCell<Stack>>,
}

pub fn parse_input(input: &str) -> Result<Input> {
    let mut is_crates = true;
    let mut crates_lines = Vec::new();
    let mut instructions = Vec::new();

    for line in input.lines() {
        if is_crates {
            if line.trim().is_empty() {
                is_crates = false;
                continue;
            }
            crates_lines.push(line);
        } else {
            instructions.push(Instruction::from(line)?);
        }
    }

    let crates = parse_crates(crates_lines)?;

    Ok(Input {
        instructions,
        crates,
    })
}

#[derive(Debug)]
struct CrateLabel {
    label: String,
    index: usize,
}

fn parse_crates(mut crates_lines: Vec<&str>) -> Result<HashMap<String, RefCell<Stack>>> {
    let labels_line = crates_lines.pop().ok_or(ParseError::InvalidLabels)?;
    let crate_labels = get_labels(labels_line);

    let mut crates = HashMap::new();

    crates_lines.reverse();
    for line in crates_lines {
        for label in crate_labels.iter() {
            if !crates.contains_key(&label.label) {
                crates.insert(label.label.clone(), RefCell::new(Stack::new()));
            }
            let crate_stack = crates
                .get_mut(&label.label)
                .ok_or(ParseError::InvalidCrateLabel(label.label.clone()))?;

            if let Some(crate_content) = line.chars().nth(label.index) {
                if crate_content.is_whitespace() {
                    continue;
                }
                crate_stack.get_mut().push(crate_content.to_string());
            }
        }
    }

    Ok(crates)
}

fn get_labels(labels_line: &str) -> Vec<CrateLabel> {
    let mut crate_labels = Vec::new();

    for (index, label) in labels_line.chars().enumerate() {
        if !label.is_whitespace() {
            crate_labels.push(CrateLabel {
                label: label.to_string(),
                index,
            });
        }
    }

    crate_labels
}
