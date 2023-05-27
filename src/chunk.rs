use crate::voxel::Voxel;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape};
use noise::{NoiseFn, Simplex};

pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug)]
pub struct Chunk {
    pub id: Entity,
    pub voxels: Vec<Voxel>,
}

pub fn generate_mesh(chunk_pos: &IVec3, chunk: &Chunk) -> (Mesh, Collider) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let chunk_offset = Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE as f32,
        chunk_pos.y as f32 * CHUNK_SIZE as f32,
        chunk_pos.z as f32 * CHUNK_SIZE as f32,
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

    // color all voxels green for now
    let colors = vec![[1.0, 1.0, 1.0, 0.0]; vertices.len()];
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));

    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();

    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    (mesh, collider)
}

pub fn generate_voxel_data(chunk_pos: IVec3) -> Vec<Voxel> {
    let mut voxels = Vec::with_capacity(CHUNK_SIZE.pow(3) as usize);
    let simplex = Simplex::new(123);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let pos = IVec3::new(
                    x + chunk_pos.x * CHUNK_SIZE,
                    y + chunk_pos.x * CHUNK_SIZE,
                    z + chunk_pos.x * CHUNK_SIZE,
                );

                let noise = simplex.get([pos.x as f64, pos.z as f64]);
                let height = y as f64 / CHUNK_SIZE as f64;
                let is_solid = noise >= height;
                voxels.push(Voxel { is_solid });
            }
        }
    }

    voxels
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