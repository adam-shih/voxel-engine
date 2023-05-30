use crate::chunk::*;
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
            render_distance: 8,
        }
    }
}

impl ChunkManager {
    pub fn load_chunk(&mut self) {
        if let Some(chunk) = self.load_queue.pop_front() {
            self.active_chunks.insert(chunk.position, chunk);
        }
    }

    pub fn unload_chunk(&mut self) {
        if let Some(chunk) = self.unload_queue.pop_front() {
            self.active_chunks.remove(&chunk.position);
        }
    }

    pub fn update(&mut self, player_position: IVec3) {
        let player_position = player_position / CHUNK_SIZE;

        self.populate_load_queue(player_position);
        self.populate_unload_queue(player_position);
    }

    fn populate_load_queue(&mut self, player_position: IVec3) {
        let min_x = player_position.x - self.render_distance;
        let max_x = player_position.x + self.render_distance;
        let min_z = player_position.z - self.render_distance;
        let max_z = player_position.z + self.render_distance;

        for x in min_x..=max_x {
            for z in min_z..=max_z {
                let chunk_pos = IVec3::new(x, 0, z);

                if self.active_chunks.contains_key(&chunk_pos) {
                    continue;
                }

                let chunk = Chunk::new(chunk_pos);
                self.load_queue.push_back(chunk);
            }
        }
    }

    fn populate_unload_queue(&mut self, player_position: IVec3) {
        let mut keys_to_remove = Vec::new();

        for chunk_pos in self.active_chunks.keys() {
            if (player_position.x - chunk_pos.x).abs() > self.render_distance
                || (player_position.z - chunk_pos.z).abs() > self.render_distance
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
        chunk_manager.update(player_position.translation.as_ivec3());
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
        let mesh = chunk.mesh_data.create_mesh();

        let id = commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::GREEN.into()),
                ..default()
            })
            .id();

        chunk_manager.load_chunk();
        chunk_entity_map.0.insert(chunk.position, id);
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
