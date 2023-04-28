use crate::voxel::{generate_voxel_data, Chunk};
use bevy::prelude::*;
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::HashMap;

const RENDER_DISTANCE: u32 = 5;

#[derive(Resource, Debug)]
pub struct ChunkManager {
    loaded: HashMap<IVec3, Chunk>,
}

impl ChunkManager {}

impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            loaded: HashMap::default(),
        }
    }
}

pub fn load_chunks(
    player_pos: Query<&Transform, With<FlyCam>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let pos = player_pos.get_single().unwrap().translation / 32.;
    let pos = pos.ceil().as_ivec3();

    for x in (pos.x - 5)..(pos.x + 5) {
        for y in (pos.y - 5)..(pos.y + 5) {
            let chunk_pos = IVec3::new(x, y, 0);
            chunk_manager.loaded.insert(
                chunk_pos,
                Chunk {
                    voxels: generate_voxel_data(chunk_pos),
                },
            );
        }
    }

    dbg!(chunk_manager);
}
