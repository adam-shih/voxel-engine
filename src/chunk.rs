use crate::voxel::VoxelData;
use bevy::prelude::*;

pub const CHUNK_SIZE: i32 = 8;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxel_data: VoxelData,
    pub position: IVec3,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        let voxel_data = VoxelData::generate_height_map(position);

        Self {
            voxel_data,
            position,
        }
    }
}
