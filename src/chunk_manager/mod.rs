use crate::{chunk::*, voxel::Voxel};
use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

pub mod plugin;

#[derive(Resource, Debug)]
pub struct ChunkManager {
    active_chunks: HashMap<IVec3, Chunk>,
    load_queue: VecDeque<Chunk>,
    unload_queue: VecDeque<Chunk>,
    reload_queue: VecDeque<Chunk>,
    render_distance: i32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            active_chunks: HashMap::new(),
            load_queue: VecDeque::new(),
            unload_queue: VecDeque::new(),
            reload_queue: VecDeque::new(),
            render_distance: 8,
        }
    }
}

impl ChunkManager {
    pub fn get_voxel_at_global_position(&self, global_pos: IVec3) -> Option<&Voxel> {
        let chunk_pos = (global_pos.as_vec3() / CHUNK_SIZE as f32).floor() * CHUNK_SIZE as f32;
        let relative_voxel_pos = (global_pos % CHUNK_SIZE).abs();

        if let Some(chunk) = self.active_chunks.get(&chunk_pos.as_ivec3()) {
            return chunk.voxel_data.voxels.get(&relative_voxel_pos);
        }

        None
    }

    // When loading a chunk, we need to add surrounding chunks to the
    // reload_queue so that the mesh of those chunks can be updated to
    // fit seamlessly to the newly loaded chunk.
    pub fn load_chunk(&mut self) {
        if let Some(chunk) = self.load_queue.pop_front() {
            for x in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && z == 0 {
                        continue;
                    }

                    let chunk_x = chunk.position.x + CHUNK_SIZE * x;
                    let chunk_z = chunk.position.z + CHUNK_SIZE * z;

                    if let Some(other_chunk) =
                        self.active_chunks.get(&IVec3::new(chunk_x, 0, chunk_z))
                    {
                        self.reload_queue.push_back(other_chunk.clone());
                    }
                }
            }

            self.active_chunks.insert(chunk.position, chunk);
        }
    }

    pub fn unload_chunk(&mut self) {
        if let Some(chunk) = self.unload_queue.pop_front() {
            self.active_chunks.remove(&chunk.position);
        }
    }

    pub fn update(&mut self, player_chunk_position: IVec3) {
        self.populate_load_queue(player_chunk_position);
        self.populate_unload_queue(player_chunk_position);
    }

    fn populate_load_queue(&mut self, player_chunk_position: IVec3) {
        for x in -self.render_distance..=self.render_distance {
            for z in -self.render_distance..=self.render_distance {
                let mut chunk_pos = player_chunk_position + (IVec3::new(x, 0, z) * CHUNK_SIZE);
                chunk_pos.y = 0;

                if self.active_chunks.contains_key(&chunk_pos) {
                    continue;
                }

                let chunk = Chunk::new(chunk_pos);
                self.load_queue.push_back(chunk);
            }
        }
    }

    fn populate_unload_queue(&mut self, player_chunk_position: IVec3) {
        let mut keys_to_remove = Vec::new();

        for chunk_pos in self.active_chunks.keys() {
            if ((player_chunk_position.x / CHUNK_SIZE) - (chunk_pos.x / CHUNK_SIZE)).abs()
                > self.render_distance
                || ((player_chunk_position.z / CHUNK_SIZE) - (chunk_pos.z / CHUNK_SIZE)).abs()
                    > self.render_distance
            {
                keys_to_remove.push(chunk_pos.clone());
            }
        }

        for chunk_pos in keys_to_remove {
            if let Some(chunk) = self.active_chunks.remove(&chunk_pos) {
                self.unload_queue.push_back(chunk);
            }
        }
    }
}
