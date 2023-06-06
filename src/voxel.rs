use std::collections::HashMap;

use crate::chunk::CHUNK_SIZE;
use bevy::prelude::*;
use noise::{NoiseFn, Simplex};

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

#[derive(Debug, Clone)]
pub struct VoxelData {
    // pub voxels: Vec<Voxel>,
    pub voxels: HashMap<IVec3, Voxel>,
}

impl VoxelData {
    pub fn coords_to_index(&self, x: i32, y: i32, z: i32) -> usize {
        (x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE) as usize
    }

    pub fn index_to_coords(&self, i: usize) -> (i32, i32, i32) {
        let x = i as i32 % CHUNK_SIZE;
        let y = (i as i32 / CHUNK_SIZE) % CHUNK_SIZE;
        let z = i as i32 / (CHUNK_SIZE * CHUNK_SIZE);

        (x, y, z)
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<&Voxel> {
        let pos = IVec3::new(x, y, z);
        self.voxels.get(&pos)
    }

    // TODO: Landscape generation
    pub fn generate_height_map(chunk_position: IVec3) -> Self {
        let mut voxels = HashMap::new();
        let simplex = Simplex::new(42);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let relative_voxel_position = IVec3::new(x, y, z);
                    let global_voxel_position =
                        relative_voxel_position + chunk_position * CHUNK_SIZE;

                    let n = (simplex.get([
                        0.009 * global_voxel_position.x as f64,
                        0.009 * global_voxel_position.z as f64,
                    ]) + 1.0)
                        / 2.0;

                    let elevation = n.powf(0.44);

                    let height = y as f64 / CHUNK_SIZE as f64;

                    if height < elevation {
                        voxels.insert(
                            relative_voxel_position,
                            Voxel {
                                is_active: true,
                                block_type: BlockType::Grass,
                            },
                        );
                    } else {
                        voxels.insert(relative_voxel_position, Voxel::default());
                    }
                }
            }
        }

        Self { voxels }
    }

    pub fn generate_sphere() -> Self {
        let mut voxels = HashMap::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if ((x - CHUNK_SIZE / 2) as f32 * (x - CHUNK_SIZE / 2) as f32
                        + (y - CHUNK_SIZE / 2) as f32 * (y - CHUNK_SIZE / 2) as f32
                        + (z - CHUNK_SIZE / 2) as f32 * (z - CHUNK_SIZE / 2) as f32)
                        .sqrt()
                        <= (CHUNK_SIZE / 2) as f32
                    {
                        voxels.insert(
                            IVec3::new(x, y, z),
                            Voxel {
                                is_active: true,
                                block_type: BlockType::Grass,
                            },
                        );
                    } else {
                        voxels.insert(IVec3::new(x, y, z), Voxel::default());
                    }
                }
            }
        }

        Self { voxels }
    }
}
