use crate::voxel::BlockType;
use crate::{mesh::MeshData, voxel::Voxel};
use bevy::prelude::*;
use noise::{NoiseFn, Simplex};

pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxel_data: VoxelData,
    pub mesh_data: MeshData,
    pub position: IVec3,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        let voxel_data = VoxelData::generate_height_map(position);
        let mesh_data = MeshData::generate(position, &voxel_data);

        Self {
            voxel_data,
            mesh_data,
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoxelData {
    pub voxels: Vec<Voxel>,
}

impl VoxelData {
    // TODO: Landscape generation
    pub fn generate_height_map(chunk_position: IVec3) -> Self {
        let mut voxels = Vec::new();
        let simplex = Simplex::new(42);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let global_voxel_position = IVec3::new(x, y, z) + chunk_position * CHUNK_SIZE;

                    let n = (simplex.get([
                        0.009 * global_voxel_position.x as f64,
                        0.009 * global_voxel_position.z as f64,
                    ]) + 1.0)
                        / 2.0;

                    let elevation = n.powf(0.44);

                    let height = y as f64 / CHUNK_SIZE as f64;

                    if height < elevation {
                        voxels.push(Voxel {
                            is_active: true,
                            block_type: BlockType::Grass,
                        });
                    } else {
                        voxels.push(Voxel::default());
                    }
                }
            }
        }

        Self { voxels }
    }

    pub fn generate_sphere() -> Self {
        let mut voxels = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if ((x - CHUNK_SIZE / 2) as f32 * (x - CHUNK_SIZE / 2) as f32
                        + (y - CHUNK_SIZE / 2) as f32 * (y - CHUNK_SIZE / 2) as f32
                        + (z - CHUNK_SIZE / 2) as f32 * (z - CHUNK_SIZE / 2) as f32)
                        .sqrt()
                        <= (CHUNK_SIZE / 2) as f32
                    {
                        voxels.push(Voxel {
                            is_active: true,
                            block_type: BlockType::Grass,
                        });
                    } else {
                        voxels.push(Voxel::default());
                    }
                }
            }
        }

        Self { voxels }
    }
}
