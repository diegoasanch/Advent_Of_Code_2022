use anyhow::Result;
use core::fmt;

#[derive(Debug)]
pub enum InstructionError {
    InvalidAmount,
    WrongNumberOfArguments(usize),
}

impl std::error::Error for InstructionError {}

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::InvalidAmount => "Invalid \"amount\"".to_string(),
            Self::WrongNumberOfArguments(args) => {
                format!("Wrong number of arguments, found {args}, expected 6")
            }
        };

        write!(f, "{error}")
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub amount: i32,
    pub from: String,
    pub to: String,
}

impl Instruction {
    pub fn from(input: &str) -> Result<Self> {
        let parts = input.split_ascii_whitespace().collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(InstructionError::WrongNumberOfArguments(parts.len()))?;
        }

        let amount = match parts[1].parse::<i32>() {
            Ok(amount) => amount,
            Err(_) => return Err(InstructionError::InvalidAmount)?,
        };

        let from = parts[3].to_string();

        let to = parts[5].to_string();

        Ok(Self { amount, from, to })
    }
}
