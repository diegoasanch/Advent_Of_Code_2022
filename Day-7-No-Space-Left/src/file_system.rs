use crate::directory::Directory;
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
        Self {
            root: Rc::new(Directory::new("/", None)),
            current_directory: (),
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
