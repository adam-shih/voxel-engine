use bevy::prelude::*;
use std::collections::HashMap;

const CHUNK_SIZE: i32 = 32;

#[derive(Debug)]
pub struct Voxel {
    pub is_solid: bool,
}

#[derive(Debug)]
pub struct Chunk {
    pub voxels: Vec<Voxel>,
}

#[derive(Debug)]
pub struct ChunkMap {
    pub map: HashMap<IVec3, Chunk>,
}

pub fn generate_voxel_data(chunk_pos: IVec3) -> Vec<Voxel> {
    let mut voxels = Vec::with_capacity(CHUNK_SIZE.pow(3) as usize);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let _pos = IVec3::new(
                    x + chunk_pos.x * CHUNK_SIZE,
                    y + chunk_pos.x * CHUNK_SIZE,
                    z + chunk_pos.x * CHUNK_SIZE,
                );
                voxels.push(Voxel { is_solid: true });
            }
        }
    }

    voxels
}

pub fn generate_mesh(chunk_map: &HashMap<IVec3, Chunk>) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (chunk_pos, chunk) in chunk_map.iter() {
        let chunk_offset = Vec3::new(
            chunk_pos.x as f32 * 1.0,
            chunk_pos.y as f32 * 1.0,
            chunk_pos.z as f32 * 1.0,
        );

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = (x + y * CHUNK_SIZE + z * CHUNK_SIZE.pow(2)) as usize;

                    if !chunk.voxels[index].is_solid {
                        continue;
                    }

                    let pos = Vec3::new(
                        x as f32 + chunk_offset.x,
                        y as f32 + chunk_offset.y,
                        z as f32 + chunk_offset.z,
                    );

                    let cube_vertices = generate_cube_vertices(pos);
                    let cube_indices = generate_cube_indices(vertices.len() as u32);

                    vertices.extend(cube_vertices);
                    indices.extend(cube_indices);
                }
            }
        }
    }

    // (vertices, indices, colors, uvs, normals)
    (vertices, indices)
}

fn generate_cube_vertices(pos: Vec3) -> Vec<[f32; 3]> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    // 8 points of cube
    vec![
        [x + 0.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 1.0, z + 0.0],
        [x + 0.0, y + 1.0, z + 0.0],
        [x + 0.0, y + 0.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 1.0],
        [x + 0.0, y + 0.0, z + 1.0],
    ]
}

#[rustfmt::skip]
fn generate_cube_indices(start_index: u32) -> Vec<u32> {
    // triangles that make up mesh
    // clockwise order gives orthogonal normal
    vec![
        // top
        0, 1, 2, 2, 3, 0, 

        // bottom
        5, 7, 4, 5, 6, 7, 

        // left
        7, 0, 4, 4, 0, 3, 

        // right 
        6, 5, 1, 1, 5, 2, 

        // front
        7, 1, 0, 7, 6, 1,

        // back
        5, 4, 3, 3, 2, 5,
    ]
    .iter()
    .map(|index| index + start_index)
    .collect()
}
