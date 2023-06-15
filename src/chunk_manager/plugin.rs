use crate::{chunk::*, mesh::MeshData};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::HashMap;

use super::ChunkManager;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .init_resource::<ChunkEntityMap>()
            .add_system(spawn_chunks)
            .add_system(despawn_chunks)
            .add_system(reload_chunks)
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

pub fn reload_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    chunk_entity_map: ResMut<ChunkEntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let reload_queue = chunk_manager.reload_queue.clone();

    for chunk in reload_queue {
        chunk_manager.reload_queue.pop_front();

        if let Some(id) = chunk_entity_map.0.get(&chunk.position) {
            let mesh_data = MeshData::generate_marching_cubes(&chunk, &chunk_manager);
            let mesh = mesh_data.create_mesh();
            // println!("{:?}", meshes.add(mesh).is_strong());
            commands
                .entity(*id)
                .insert((meshes.add(mesh), materials.add(Color::RED.into())));
        }
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
        chunk_manager.load_chunk();

        let mesh_data = MeshData::generate_marching_cubes(&chunk, &chunk_manager);
        let mesh = mesh_data.create_mesh();

        let id = commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::GREEN.into()),
                ..default()
            })
            .id();

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
