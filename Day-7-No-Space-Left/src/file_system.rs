use crate::{directory::Directory, inode::INode};
use anyhow::Result;
use std::rc::Rc;

pub struct FileSystem {
    root: Rc<Directory>,
    current_directory: Rc<Directory>,
}

enum DirectoryChange<'a> {
    Root,
    Parent,
    Relative(&'a str),
}

enum DirectoryChangeError {
    NoParent,
}

impl FileSystem {
    pub fn new() -> Self {
        // Root directory of the file system
        let root_dir = Directory::new("/", None);
        let root = Rc::new(root_dir);

        let current_directory = Rc::clone(&root);

        Self {
            root,
            current_directory,
        }
    }

    pub fn change_directory(&mut self, to: DirectoryChange) -> Result<()> {
        match to {
            DirectoryChange::Parent => {
                match self.current_directory.parent() {
                    Some(parent) => {
                        self.current_directory = parent;
                    }
                    None => Err(DirectoryChangeError::NoParent),
                }
                self.current_directory = parent_dir;
            }
        };

        Ok(())
    }
}

impl INode for FileSystem {
    fn name(&self) -> &str {
        self.root.name()
    }

    fn size(&self) -> u32 {
        self.root.size()
    }

    fn is_directory(&self) -> bool {
        true
    }
}
