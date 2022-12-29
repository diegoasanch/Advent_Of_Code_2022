pub trait INode {
    fn size(&self) -> u32;
    fn name(&self) -> &str;
    fn is_directory(&self) -> bool;
}
