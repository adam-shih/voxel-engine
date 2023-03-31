use bevy::prelude::{IVec3, Vec3};
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use rand::prelude::*;
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
    // TODO: make this noise actually work

    let mut rng = rand::thread_rng();
    // let mut noise = FastNoise::seeded(rng.gen());
    // noise.set_noise_type(NoiseType::PerlinFractal);
    // noise.set_fractal_type(FractalType::FBM);
    // noise.set_fractal_octaves(5);
    // noise.set_fractal_lacunarity(2.0);
    // noise.set_frequency(2.0);

    let mut voxels = Vec::with_capacity(32 * 32 * 32);

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let pos = IVec3::new(
                    x + chunk_pos.x * 32,
                    y + chunk_pos.x * 32,
                    z + chunk_pos.x * 32,
                );
                voxels.push(Voxel { is_solid: rng.gen() });
            }
        }
    }

    voxels
}

pub fn generate_mesh(chunk_map: &HashMap<IVec3, Chunk>) -> (Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 4]>, Vec<[f32; 2]>, Vec<[f32; 3]>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut colors = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();

    for (chunk_pos, chunk) in chunk_map.iter() {
        let chunk_offset = Vec3::new(
            chunk_pos.x as f32 * 32.0,
            chunk_pos.y as f32 * 32.0,
            chunk_pos.z as f32 * 32.0,
        );

        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    let index = x + y * 32 + z * 1024;

                    if !chunk.voxels[index].is_solid {
                        continue;
                    }

                    let pos = Vec3::new(
                        x as f32 + chunk_offset.x,
                        y as f32 + chunk_offset.y,
                        z as f32 + chunk_offset.z
                    );

                    let cube_vertices = generate_cube_vertices(pos);
                    let cube_indices = generate_cube_indices(vertices.len() as u32);
                    println!("Vertices len: {}", cube_vertices.len());
                    vertices.extend(cube_vertices);
                    indices.extend(cube_indices);
                    
                    for _ in 0..3 {
                        colors.extend([[0.0, 1.0, 0.0, 1.0]; 8]);
                        uvs.extend([[1.0, 0.0]; 8]);
                        normals.extend([[1.0, 0.0, 0.0]; 8]);
                    }
                }
            }
        }
    }

    (vertices, indices, colors, uvs, normals)
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
    vec![
        0, 1, 2, 2, 3, 0,
        5, 7, 6, 5, 4, 7,
        7, 0, 4, 4, 0, 3,
        6, 5, 1, 1, 5, 2,
        7, 1, 0, 7, 6, 1,
        5, 4, 3, 3, 2, 5,
    ]
    .iter()
    .map(|index| index + start_index)
    .collect()
}