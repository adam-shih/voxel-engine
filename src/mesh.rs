use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::chunk_manager::ChunkManager;
use crate::tables::TRIANGULATION;
use crate::voxel::VoxelData;
use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};

#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn create_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();

        mesh
    }

    pub fn generate_marching_cubes(chunk: &Chunk, chunk_manager: &ChunkManager) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // let chunk_offset = (chunk_position * CHUNK_SIZE).as_vec3();

        for x in -1..CHUNK_SIZE {
            for y in -1..CHUNK_SIZE {
                for z in -1..CHUNK_SIZE {
                    let mut case = 0;
                    let relative_voxel_position = IVec3::new(x, y, z);
                    let global_voxel_position = relative_voxel_position + chunk.position;

                    let global_cube_vertices =
                        generate_cube_vertices(global_voxel_position.as_vec3());
                    let cube_vertices = generate_cube_vertices(relative_voxel_position.as_vec3());

                    for (i, vertex) in cube_vertices.iter().enumerate() {
                        let pos = IVec3::new(vertex[0] as i32, vertex[1] as i32, vertex[2] as i32);

                        if let Some(voxel) = chunk.voxel_data.voxels.get(&pos) {
                            if voxel.is_active {
                                case |= 1 << i;
                            }
                        } else {
                            println!("yes");
                            let global_pos = IVec3::new(
                                global_cube_vertices[i][0] as i32,
                                global_cube_vertices[i][1] as i32,
                                global_cube_vertices[i][2] as i32,
                            );

                            if let Some(voxel) =
                                chunk_manager.get_voxel_at_global_position(global_pos)
                            {
                                if voxel.is_active {
                                    case |= 1 << i;
                                }
                            }
                        }
                    }

                    // lookup case in table to get triangles
                    let triangles = TRIANGULATION[case]
                        .clone()
                        .iter()
                        .filter(|i| **i != -1)
                        .map(|i| *i as u32 + vertices.len() as u32)
                        .collect::<Vec<_>>();

                    vertices.extend(generate_cube_edges(global_voxel_position.as_vec3()));
                    indices.extend(triangles);
                }
            }
        }

        Self { vertices, indices }
    }

    pub fn generate(chunk_position: IVec3, voxel_data: &VoxelData) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let chunk_offset = (chunk_position * CHUNK_SIZE).as_vec3();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if let Some(voxel) = voxel_data.get(x, y, z) {
                        if voxel.is_active {
                            continue;
                        }
                    }

                    let global_voxel_pos = Vec3::new(
                        x as f32 + chunk_offset.x,
                        y as f32 + chunk_offset.y,
                        z as f32 + chunk_offset.z,
                    );

                    let cube_vertices = generate_cube_vertices(global_voxel_pos);
                    let cube_indices = generate_cube_indices(vertices.len() as u32);

                    vertices.extend(cube_vertices);
                    indices.extend(cube_indices);
                }
            }
        }

        Self { vertices, indices }
    }
}

fn generate_cube_vertices(pos: Vec3) -> Vec<[f32; 3]> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    // 8 points of cube
    vec![
        [x + 0.0, y + 0.0, z + 0.0],
        [x + 0.0, y + 1.0, z + 0.0],
        [x + 1.0, y + 1.0, z + 0.0],
        [x + 1.0, y + 0.0, z + 0.0],
        [x + 0.0, y + 0.0, z + 1.0],
        [x + 0.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 1.0, z + 1.0],
        [x + 1.0, y + 0.0, z + 1.0],
    ]
}

fn generate_cube_edges(pos: Vec3) -> Vec<[f32; 3]> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    vec![
        [x + 0.0, y + 0.5, z + 0.0],
        [x + 0.5, y + 1.0, z + 0.0],
        [x + 1.0, y + 0.5, z + 0.0],
        [x + 0.5, y + 0.0, z + 0.0],
        [x + 0.0, y + 0.5, z + 1.0],
        [x + 0.5, y + 1.0, z + 1.0],
        [x + 1.0, y + 0.5, z + 1.0],
        [x + 0.5, y + 0.0, z + 1.0],
        [x + 0.0, y + 0.0, z + 0.5],
        [x + 0.0, y + 1.0, z + 0.5],
        [x + 1.0, y + 1.0, z + 0.5],
        [x + 1.0, y + 0.0, z + 0.5],
    ]
}

fn generate_cube_indices(start_index: u32) -> Vec<u32> {
    // indices of points that make up triangles
    vec![
        1, 5, 2, 5, 6, 2, // top
        3, 4, 0, 3, 7, 4, // bottom
        5, 1, 4, 4, 1, 0, // left
        2, 6, 7, 2, 7, 3, // right
        1, 2, 3, 3, 0, 1, // front
        6, 5, 7, 5, 4, 7, // back
    ]
    .iter()
    .map(|index| index + start_index)
    .collect()
}
