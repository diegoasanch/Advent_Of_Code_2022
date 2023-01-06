use crate::inode::{INode, INodeRef, INodeWeak};
use anyhow::Result;

use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Directory {
    name: String,
    parent: Option<INodeWeak>,
    items: Vec<INodeRef>,
}

impl Directory {
    pub fn new(name: &str, parent: Option<INodeWeak>) -> Self {
        Self {
            name: name.to_string(),
            parent,
            items: Vec::new(),
        }
    }
}

impl INode for Directory {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u32 {
        self.items.iter().map(|item| item.borrow().size()).sum()
    }

    fn is_directory(&self) -> bool {
        true
    }

    fn parent(&self) -> Option<INodeWeak> {
        self.parent.as_ref().map(|parent| Weak::clone(parent))
    }

    fn set_parent(&mut self, parent: Option<INodeWeak>) {
        self.parent = parent;
    }

    fn items(&self) -> Option<Vec<INodeWeak>> {
        Some(self.items.iter().map(|item| Rc::downgrade(item)).collect())
    }

    fn add_item(&mut self, item: INodeRef) -> Result<()> {
        self.items.push(item);
        Ok(())
    }
}
