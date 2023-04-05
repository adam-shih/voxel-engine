use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use rand::prelude::*;
use std::collections::HashMap;

const CHUNK_SIZE: i32 = 1;

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
    // TODO: make this noise actually work

    let mut rng = rand::thread_rng();
    // let mut noise = FastNoise::seeded(rng.gen());
    // noise.set_noise_type(NoiseType::PerlinFractal);
    // noise.set_fractal_type(FractalType::FBM);
    // noise.set_fractal_octaves(5);
    // noise.set_fractal_lacunarity(2.0);
    // noise.set_frequency(2.0);

    let mut voxels = Vec::with_capacity(1 * 1 * 1);

    for x in 0..1 {
        for y in 0..1 {
            for z in 0..1 {
                let pos = IVec3::new(
                    x + chunk_pos.x * 1,
                    y + chunk_pos.x * 1,
                    z + chunk_pos.x * 1,
                );
                voxels.push(Voxel { is_solid: true });
            }
        }
    }

    voxels
}

pub fn generate_mesh(
    chunk_map: &HashMap<IVec3, Chunk>,
) -> (Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 3]>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    for (chunk_pos, chunk) in chunk_map.iter() {
        let chunk_offset = Vec3::new(
            chunk_pos.x as f32 * 1.0,
            chunk_pos.y as f32 * 1.0,
            chunk_pos.z as f32 * 1.0,
        );

        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    let index = x + y * 1 + z * 1024;

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

                    // Calculate the normal vector for each face
                    let normals_per_face = [
                        [0.0, 0.0, 1.0],  // front
                        [0.0, 0.0, -1.0], // back
                        [-1.0, 0.0, 0.0], // left
                        [1.0, 0.0, 0.0],  // right
                        [0.0, 1.0, 0.0],  // top
                        [0.0, -1.0, 0.0], // bottom
                    ];

                    // Add normals to each vertex of the cube
                    for i in 0..cube_vertices.len() {
                        let normal = normals_per_face[i / 4]; // Each face has 4 vertices
                        normals.push(normal);
                        vertices.push(cube_vertices[i]);
                    }

                    indices.extend(cube_indices);
                }
            }
        }
    }

    // (vertices, indices, colors, uvs, normals)
    (vertices, indices, normals)
}

fn generate_cube_vertices(pos: Vec3) -> Vec<[f32; 3]> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    vec![
        // top face
        [x + 0.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 1.0, z + 0.0],
        [x + 0.0, y + 1.0, z + 0.0],
        // bottom face
        [x + 0.0, y + 0.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 1.0],
        [x + 0.0, y + 0.0, z + 1.0],
        // left face
        [x + 0.0, y + 1.0, z + 0.0],
        [x + 0.0, y + 1.0, z + 1.0],
        [x + 0.0, y + 0.0, z + 1.0],
        [x + 0.0, y + 0.0, z + 0.0],
        // right face
        [x + 1.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 1.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 1.0],
        // front face
        [x + 0.0, y + 1.0, z + 0.0],
        [x + 1.0, y + 1.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 0.0],
        [x + 0.0, y + 0.0, z + 0.0],
        // back face
        [x + 1.0, y + 1.0, z + 1.0],
        [x + 0.0, y + 1.0, z + 1.0],
        [x + 0.0, y + 0.0, z + 1.0],
        [x + 1.0, y + 0.0, z + 1.0],
    ]
}

fn generate_cube_indices(start_index: u32) -> Vec<u32> {
    // vec![
    //     0, 1, 2, 2, 3, 0, 5, 7, 6, 5, 4, 7, 7, 0, 4, 4, 0, 3, 6, 5, 1, 1, 5, 2, 7, 1, 0, 7, 6, 1,
    //     5, 4, 3, 3, 2, 5,
    // ]
    // .iter()
    // .map(|index| index + start_index)
    // .collect()
    vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // left
        12, 13, 14, 14, 15, 12, // right
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ]
    .iter()
    .map(|index| index + start_index)
    .collect()
}
