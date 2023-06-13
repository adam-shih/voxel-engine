use crate::{chunk::*, mesh::MeshData, voxel::Voxel};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::{HashMap, VecDeque};

#[derive(Resource, Debug)]
pub struct ChunkManager {
    active_chunks: HashMap<IVec3, Chunk>,
    load_queue: VecDeque<Chunk>,
    unload_queue: VecDeque<Chunk>,
    render_distance: i32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            active_chunks: HashMap::new(),
            load_queue: VecDeque::new(),
            unload_queue: VecDeque::new(),
            render_distance: 4,
        }
    }
}

impl ChunkManager {
    pub fn get_voxel_at_global_position(&self, global_pos: IVec3) -> Option<&Voxel> {
        // let chunk_pos = global_pos / CHUNK_SIZE;
        let chunk_pos = (global_pos.as_vec3() / CHUNK_SIZE as f32).floor() * CHUNK_SIZE as f32;
        let relative_voxel_pos = global_pos % CHUNK_SIZE;

        if let Some(chunk) = self.active_chunks.get(&chunk_pos.as_ivec3()) {
            return chunk.voxel_data.voxels.get(&relative_voxel_pos);
        }

        None
    }

    pub fn load_chunk(&mut self) -> Option<Vec<Chunk>> {
        let mut reload_queue = Vec::new();

        if let Some(chunk) = self.load_queue.pop_front() {
            // check for chunks to update around new load
            for x in chunk.position.x - 1..=chunk.position.x + 1 {
                for z in chunk.position.z - 1..=chunk.position.z + 1 {
                    if let Some(other_chunk) = self.active_chunks.get(&IVec3::new(x, 0, z)) {
                        reload_queue.push(other_chunk.clone());
                    }
                }
            }

            self.active_chunks.insert(chunk.position, chunk);
            return Some(reload_queue);
        }

        None
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
        // let min_x = player_chunk_position.x - self.render_distance;
        // let max_x = player_chunk_position.x + self.render_distance;
        // let min_z = player_chunk_position.z - self.render_distance;
        // let max_z = player_chunk_position.z + self.render_distance;

        for x in -self.render_distance..=self.render_distance {
            for z in -self.render_distance..=self.render_distance {
                // let chunk_pos = IVec3::new(x, 0, z);
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

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .init_resource::<ChunkEntityMap>()
            .add_system(spawn_chunks)
            .add_system(despawn_chunks)
            .add_system(update_chunk_manager);
    }
}

#[derive(Resource)]
pub struct ChunkEntityMap(HashMap<IVec3, Entity>);

impl Default for ChunkEntityMap {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

pub fn update_chunk_manager(
    mut chunk_manager: ResMut<ChunkManager>,
    player_pos_query: Query<&Transform, With<FlyCam>>,
) {
    if let Ok(player_position) = player_pos_query.get_single() {
        let player_chunk_position = player_position.translation / CHUNK_SIZE as f32;
        let player_chunk_position = player_chunk_position.floor().as_ivec3() * CHUNK_SIZE;
        chunk_manager.update(player_chunk_position);
    }
}

pub fn spawn_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_entity_map: ResMut<ChunkEntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let load_queue = chunk_manager.load_queue.clone();

    for chunk in load_queue {
        let reload_queue = chunk_manager.load_chunk();

        let mesh_data = MeshData::generate_marching_cubes(chunk.position, &chunk_manager);
        let mesh = mesh_data.create_mesh();

        let id = commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::GREEN.into()),
                ..default()
            })
            .id();

        chunk_entity_map.0.insert(chunk.position, id);

        if let Some(q) = reload_queue {
            for chunk in q {
                let mesh_data = MeshData::generate_marching_cubes(chunk.position, &chunk_manager);
                let mesh = mesh_data.create_mesh();
                if let Some(id) = chunk_entity_map.0.get(&chunk.position) {
                    commands.entity(*id).insert(meshes.add(mesh));
                }
            }
        }
    }
}

pub fn despawn_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_entity_map: ResMut<ChunkEntityMap>,
) {
    let unload_queue = chunk_manager.unload_queue.clone();

    for chunk in unload_queue {
        if let Some(id) = chunk_entity_map.0.remove(&chunk.position) {
            commands.entity(id).despawn();
        }

        chunk_manager.unload_chunk();
    }
}
