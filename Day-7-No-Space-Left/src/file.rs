use crate::inode::INode;

struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn new(name: String, size: u32) -> Self {
        Self { name, size }
    }
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
}
