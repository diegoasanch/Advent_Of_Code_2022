use anyhow::Result;
use std::fmt;

#[derive(Debug)]
pub enum StackError {
    StackEmpty,
}
impl std::error::Error for StackError {}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::StackEmpty => String::from("Stack is empty"),
        };
        write!(f, "{error}")
    }
}

#[derive(Debug)]
pub struct Stack {
    items: Vec<String>,
}

impl Stack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, value: String) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<String> {
        match self.items.last() {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    /// Moves an item from `self` to `destination`
    pub fn move_item(&mut self, destination: &mut Self) -> Result<()> {
        match self.pop() {
            Some(item) => {
                destination.push(item);
                Ok(())
            }
            None => Err(StackError::StackEmpty)?,
        }
    }

    /// Moves `amount` items from `self` to `destination`
    pub fn move_amount(&mut self, destination: &mut Self, amount: usize) -> Result<()> {
        let mut temp_stack = Self::new();
        for _ in 0..amount {
            self.move_item(&mut temp_stack)?;
        }
        for _ in 0..amount {
            temp_stack.move_item(destination)?;
        }
        Ok(())
    }
}
