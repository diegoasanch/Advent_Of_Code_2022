use core::fmt::Debug;

use anyhow::Result;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type INodeRef = Rc<RefCell<dyn INode>>;
pub type INodeWeak = Weak<RefCell<dyn INode>>;

/// File system inode
pub trait INode {
    /// Returns the `size` of the inode in **bytes**
    fn size(&self) -> u32;

    /// Returns the name of the inode
    fn name(&self) -> &str;

    /// Returns true if the inode is a directory
    fn is_directory(&self) -> bool;

    /// Returns a Weak reference to the parent inode
    fn parent(&self) -> Option<INodeWeak>;

    /// Sets the parent inode
    fn set_parent(&mut self, parent: Option<INodeWeak>);

    /// Adds an item to the inode
    fn add_item(&mut self, item: INodeRef) -> Result<()>;

    /// List of items in the inode
    fn items(&self) -> Option<Vec<INodeWeak>>;

    /// Find a child inode by name
    fn find_item(&self, name: &str) -> Option<INodeRef> {
        for item in self.items()? {
            if let Some(item) = item.upgrade() {
                if item.borrow().name() == name {
                    return Some(item);
                }
            }
        }
        None
    }

    /// Pretty prints the inode
    /// Recursively prints the items of the inode
    /// if it is a directory
    fn print_tree(&self, depth: usize) {
        let indent = " ".repeat(depth);

        let prefix = if self.is_directory() {
            "└─"
        } else {
            "├─"
        };
        println!("{}{}{} --- {}", indent, prefix, self.name(), self.size());

        if let Some(items) = self.items() {
            for item in items {
                match item.upgrade() {
                    Some(item) => item.borrow().print_tree(depth + 2),
                    None => println!("{}<DEAD>", indent),
                }
            }
        }
    }
}

impl Debug for dyn INode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INode {{ name: {}, size: {}, is_directory: {} }}",
            self.name(),
            self.size(),
            self.is_directory()
        )
    }
}
