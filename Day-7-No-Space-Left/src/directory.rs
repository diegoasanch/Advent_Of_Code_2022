use crate::inode::INode;
use std::rc::{Rc, Weak};

pub struct Directory {
    name: String,
    parent: Option<Weak<Directory>>,
    items: Vec<Rc<dyn INode>>,
}

impl Directory {
    pub fn new(name: &str, parent: Option<Weak<Directory>>) -> Self {
        Self {
            name: name.to_string(),
            parent,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Rc<dyn INode>) {
        self.items.push(item);
    }

    pub fn parent(&self) -> Option<Weak<Directory>> {
        match self.parent {
            Some(parent) => Some(Weak::clone(&parent)),
            None => None,
        }
    }
}

impl INode for Directory {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u32 {
        self.items.iter().map(|item| item.size()).sum()
    }

    fn is_directory(&self) -> bool {
        true
    }
}
