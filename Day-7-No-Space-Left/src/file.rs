use anyhow::Result;
use std::rc::Weak;
use thiserror::Error;

use crate::inode::{INode, INodeWeak};

#[derive(Error, Debug)]
#[error("Not a directory")]
struct NotADirectoryError;

pub struct File {
    name: String,
    size: u32,
    parent: Option<INodeWeak>,
}

impl File {
    pub fn new(name: &str, size: u32, parent: Option<INodeWeak>) -> Self {
        Self {
            name: name.to_string(),
            size,
            parent,
        }
    }

    // pub fn
}

impl INode for File {
    fn is_directory(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn parent(&self) -> Option<INodeWeak> {
        self.parent.as_ref().map(|parent| Weak::clone(&parent))
    }

    fn set_parent(&mut self, parent: Option<INodeWeak>) {
        self.parent = parent;
    }

    fn items(&self) -> Option<Vec<INodeWeak>> {
        None
    }

    fn add_item(&mut self, _item: crate::inode::INodeRef) -> Result<()> {
        Err(NotADirectoryError.into())
    }
}
