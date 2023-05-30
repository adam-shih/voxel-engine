#[derive(Debug, Clone)]
pub struct Voxel {
    pub is_active: bool,
    pub block_type: BlockType,
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            is_active: false,
            block_type: BlockType::Default,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Default,
    Grass,
}
