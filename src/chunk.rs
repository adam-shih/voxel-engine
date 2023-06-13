use crate::voxel::VoxelData;
use bevy::prelude::*;

pub const CHUNK_SIZE: i32 = 4;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxel_data: VoxelData,
    // pub mesh_data: MeshData,
    pub position: IVec3,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        let voxel_data = VoxelData::generate_height_map(position);
        // let voxel_data = VoxelData::generate_sphere();
        // let mesh_data = MeshData::generate(position, &voxel_data);
        // let mesh_data = MeshData::generate_marching_cubes(position, &voxel_data);

        Self {
            voxel_data,
            // mesh_data,
            position,
        }
    }
}
