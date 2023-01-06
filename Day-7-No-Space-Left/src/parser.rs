use std::{cell::RefCell, rc::Rc};

use crate::{
    directory::Directory,
    file::File,
    file_system::{DirectoryChange, FileSystem},
};
use anyhow::Result;
use thiserror::Error;

/// Parsed action from a line of input
#[derive(Debug)]
enum ParsedAction {
    Command(FileSystemCommand),
    DirectoryContent(DirectoryContent),
}

/// Content of a directory, could be a file or another directory
#[derive(Debug)]
enum DirectoryContent {
    File(FileInfo),
    Directory(DirectoryInfo),
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct DirectoryInfo {
    name: String,
}

#[derive(Debug)]
enum FileSystemCommand {
    ChangeDirectory(DirectoryChange),
    ListDirectory,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Invalid directory change: {0}")]
    InvalidDirectoryChange(String),
}

impl ParsedAction {
    fn parse(line: &str) -> Result<Self> {
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        if line_parts.len() <= 1 {
            return Err(ParseError::InvalidInput(line.to_string()).into());
        }

        match line_parts[0] {
            "$" => {
                let command = FileSystemCommand::parse(&line_parts[1..])?;
                Ok(Self::Command(command))
            }
            _ => {
                let directory_content = DirectoryContent::parse(line)?;
                Ok(Self::DirectoryContent(directory_content))
            }
        }
    }
}

impl FileSystemCommand {
    pub fn parse(command_parts: &[&str]) -> Result<Self> {
        if command_parts.len() == 0 {
            return Err(ParseError::InvalidInput(command_parts.join(" ")).into());
        }

        match command_parts[0] {
            "cd" => {
                if command_parts.len() != 2 {
                    return Err(ParseError::InvalidDirectoryChange(
                        "Missing target directory".to_string(),
                    )
                    .into());
                }

                let directory_change = match command_parts[1] {
                    "/" => DirectoryChange::Root,
                    ".." => DirectoryChange::Parent,
                    _ => DirectoryChange::Relative(command_parts[1].to_string()),
                };

                Ok(Self::ChangeDirectory(directory_change))
            }
            "ls" => Ok(Self::ListDirectory),
            _ => Err(ParseError::InvalidCommand(command_parts.join(" ")).into()),
        }
    }
}

impl DirectoryContent {
    fn parse(line: &str) -> Result<Self> {
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        if line_parts.len() != 2 {
            return Err(ParseError::InvalidInput(line.to_string()).into());
        }

        match line_parts[0] {
            "dir" => {
                let directory_info = DirectoryInfo {
                    name: line_parts[1].to_string(),
                };
                Ok(Self::Directory(directory_info))
            }
            _ => {
                let file_info = FileInfo {
                    name: line_parts[1].to_string(),
                    size: line_parts[0].parse::<u32>()?,
                };
                Ok(Self::File(file_info))
            }
        }
    }
}

pub fn parse_file_system(input: &str) -> Result<FileSystem> {
    let mut file_system = FileSystem::new();

    for line in input.lines() {
        let parsed_action = ParsedAction::parse(line)?;
        perform_fs_action(&mut file_system, &parsed_action)?;
    }

    Ok(file_system)
}

fn perform_fs_action(file_system: &mut FileSystem, action: &ParsedAction) -> Result<()> {
    match action {
        ParsedAction::Command(command) => match command {
            FileSystemCommand::ChangeDirectory(directory_change) => {
                file_system.change_directory(&directory_change)?;
            }
            FileSystemCommand::ListDirectory => {
                // file_system.list_directory
                // DO: nothing
            }
        },
        ParsedAction::DirectoryContent(directory_content) => match directory_content {
            DirectoryContent::File(file_info) => {
                let file = File::new(&file_info.name, file_info.size, None);
                file_system.add_item(Rc::new(RefCell::new(file)))?;
            }
            DirectoryContent::Directory(directory_info) => {
                let dir = Directory::new(&directory_info.name, None);
                file_system.add_item(Rc::new(RefCell::new(dir)))?;
            }
        },
    }
    Ok(())
}
