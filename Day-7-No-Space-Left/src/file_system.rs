use crate::{
    directory::Directory,
    inode::{INode, INodeRef, INodeWeak},
};
use anyhow::Result;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use thiserror::Error;

#[derive(Debug)]
pub struct FileSystem {
    root: INodeRef,
    location: INodeWeak,
}

#[derive(Debug)]
pub enum DirectoryChange {
    Root,
    Parent,
    Relative(String),
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("No parent directory")]
    NoParent,

    #[error("Current directory unset")]
    /// This error should never occur, but is here for completeness
    CurrentDirectoryUnset,

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
}

impl FileSystem {
    pub fn new() -> Self {
        // Root directory of the file system
        let root_dir = Directory::new("/", None);
        let root = Rc::new(RefCell::new(root_dir));

        let location = Rc::downgrade(&root);

        Self { root, location }
    }

    pub fn change_directory(&mut self, to: &DirectoryChange) -> Result<()> {
        match to {
            DirectoryChange::Parent => self.change_to_parent(),
            DirectoryChange::Root => self.change_to_root(),
            DirectoryChange::Relative(path) => self.change_to_relative(path),
        }
    }

    /// Sets the current directory to the parent of the current directory
    fn change_to_parent(&mut self) -> Result<()> {
        let parent = self
            .location
            .upgrade()
            .ok_or(FileSystemError::CurrentDirectoryUnset)?
            .try_borrow()?
            .parent()
            .ok_or(FileSystemError::NoParent)?;

        self.location = Weak::clone(&parent);
        Ok(())
    }

    /// Sets the current directory to the root directory
    fn change_to_root(&mut self) -> Result<()> {
        self.location = Rc::downgrade(&self.root);
        Ok(())
    }

    /// Sets the current directory to the directory at the given path
    /// relative to the current directory
    fn change_to_relative(&mut self, directory_path: &str) -> Result<()> {
        let current_inode = self
            .location
            .upgrade()
            .ok_or(FileSystemError::CurrentDirectoryUnset)?;

        let inode_items =
            current_inode
                .try_borrow()?
                .items()
                .ok_or(FileSystemError::DirectoryNotFound(
                    directory_path.to_string(),
                ))?;

        let new_location = inode_items
            .iter()
            .find(|node| {
                match node.upgrade().or(None).map(|node| {
                    node.try_borrow()
                        .ok()
                        .map(|node| node.name() == directory_path)
                }) {
                    Some(_item) => true,
                    _ => false,
                }
            })
            .ok_or(FileSystemError::DirectoryNotFound(
                directory_path.to_string(),
            ))?;

        self.location = Weak::clone(&new_location);
        Ok(())
    }

    pub fn add_item(&mut self, item: INodeRef) -> Result<()> {
        let current_inode = &self.location;

        item.try_borrow_mut()?
            .set_parent(Some(Weak::clone(current_inode)));

        current_inode
            .upgrade()
            .ok_or(FileSystemError::CurrentDirectoryUnset)?
            .try_borrow_mut()?
            .add_item(item)?;

        Ok(())
    }
}

impl INode for FileSystem {
    fn name(&self) -> &str {
        // &self.root.borrow().name().to_string()
        "file_system_root"
    }

    fn size(&self) -> u32 {
        self.root.borrow().size()
    }

    fn is_directory(&self) -> bool {
        true
    }

    fn parent(&self) -> Option<INodeWeak> {
        self.root
            .borrow()
            .parent()
            .map(|parent| Weak::clone(&parent))
    }

    fn set_parent(&mut self, parent: Option<INodeWeak>) {
        self.root.borrow_mut().set_parent(parent);
    }

    fn items(&self) -> Option<Vec<INodeWeak>> {
        self.root.borrow().items()
    }

    fn add_item(&mut self, item: INodeRef) -> Result<()> {
        self.root.borrow_mut().add_item(item)
    }
}
