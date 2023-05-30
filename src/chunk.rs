use crate::voxel::Voxel;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxel_data: VoxelData,
    pub mesh_data: MeshData,
    pub position: IVec3,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        let voxel_data = VoxelData::generate_sphere();
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
                        voxels.push(Voxel { is_solid: true });
                    } else {
                        voxels.push(Voxel { is_solid: false });
                    }
                }
            }
        }

        Self { voxels }
    }
}

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

    pub fn generate(position: IVec3, voxel_data: &VoxelData) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let chunk_offset = Vec3::new(
            position.x as f32 * CHUNK_SIZE as f32,
            position.y as f32 * CHUNK_SIZE as f32,
            position.z as f32 * CHUNK_SIZE as f32,
        );

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = (x + y * CHUNK_SIZE + z * CHUNK_SIZE.pow(2)) as usize;

                    if !voxel_data.voxels[index].is_solid {
                        continue;
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

fn generate_cube_indices(start_index: u32) -> Vec<u32> {
    // indices of points that make up triangles
    vec![
        0, 1, 2, 2, 3, 0, // top
        5, 7, 4, 5, 6, 7, // bottom
        7, 0, 4, 4, 0, 3, // left
        6, 5, 1, 1, 5, 2, // right
        7, 1, 0, 7, 6, 1, // front
        5, 4, 3, 3, 2, 5, // back
    ]
    .iter()
    .map(|index| index + start_index)
    .collect()
}
