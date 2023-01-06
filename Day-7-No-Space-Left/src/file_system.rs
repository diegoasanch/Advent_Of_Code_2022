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
    location: INodeRef,
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

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("Dead reference")]
    DeadReference,
}

impl FileSystem {
    pub fn new() -> Self {
        // Root directory of the file system
        let root_dir = Directory::new("/", None);
        let root = Rc::new(RefCell::new(root_dir));

        let location = Rc::clone(&root);

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
        let parent_weak = self
            .location
            .try_borrow()?
            .parent()
            .ok_or(FileSystemError::NoParent)?;

        let parent = Weak::upgrade(&parent_weak).ok_or(FileSystemError::DeadReference)?;

        self.location = parent;
        Ok(())
    }

    /// Sets the current directory to the root directory
    fn change_to_root(&mut self) -> Result<()> {
        self.location = Rc::clone(&self.root);
        Ok(())
    }

    /// Sets the current directory to the directory at the given path
    /// relative to the current directory
    fn change_to_relative(&mut self, directory_path: &str) -> Result<()> {
        let current_inode = &self.location;
        let new_location = current_inode.borrow().find_item(directory_path).ok_or(
            FileSystemError::DirectoryNotFound(directory_path.to_string()),
        )?;
        self.location = new_location;
        Ok(())
    }

    pub fn add_item(&mut self, item: INodeRef) -> Result<()> {
        let current_inode = &self.location;

        item.try_borrow_mut()?
            .set_parent(Some(Rc::downgrade(current_inode)));

        current_inode.try_borrow_mut()?.add_item(item)?;

        Ok(())
    }
}

impl INode for FileSystem {
    fn name(&self) -> &str {
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
