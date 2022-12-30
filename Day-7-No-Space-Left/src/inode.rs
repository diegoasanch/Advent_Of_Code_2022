/// File system inode
pub trait INode {
    /// Returns the `size` of the inode in **bytes**
    fn size(&self) -> u32;

    /// Returns the name of the inode
    fn name(&self) -> &str;

    /// Returns true if the inode is a directory
    fn is_directory(&self) -> bool;
}
