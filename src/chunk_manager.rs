use crate::{chunk::*, config::Config};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::HashMap;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>().add_systems((
            mark_chunks_to_load,
            load_chunks,
            mark_chunks_to_unload,
            unload_chunks,
        ));
    }
}

#[derive(Resource, Debug)]
pub struct ChunkManager {
    active_chunks: HashMap<IVec3, Chunk>,
    render_buffer: Vec<Chunk>,
    unload_buffer: Vec<Chunk>,
    render_distance: i32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            active_chunks: HashMap::new(),
            render_buffer: Vec::new(),
            unload_buffer: Vec::new(),
            render_distance: 8,
        }
    }
}

impl ChunkManager {
    fn populate_render_buffer(&mut self, player_position: IVec3) {
        let min_x = player_position.x - self.render_distance;
        let max_x = player_position.x + self.render_distance;
        let min_z = player_position.z - self.render_distance;
        let max_z = player_position.z + self.render_distance;

        for x in min_x..=max_x {
            for z in min_z..=max_z {}
        }
    }
}
