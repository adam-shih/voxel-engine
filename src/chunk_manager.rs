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
    unloaded: HashMap<IVec3, Chunk>,
    loaded: HashMap<IVec3, Chunk>,
}

impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            unloaded: HashMap::default(),
            loaded: HashMap::default(),
        }
    }
}

#[derive(Component)]
pub struct WantsToLoad(pub IVec3);

#[derive(Component)]
pub struct WantsToUnload(pub IVec3);

pub fn mark_chunks_to_unload(
    mut commands: Commands,
    config: Res<Config>,
    player_pos: Query<&Transform, With<FlyCam>>,
    chunk_manager: ResMut<ChunkManager>,
) {
    let player_chunk_pos = player_pos.single().translation / CHUNK_SIZE as f32;
    let player_chunk_pos = player_chunk_pos.floor().as_ivec3();

    for (chunk_pos, chunk) in chunk_manager.loaded.iter() {
        if (player_chunk_pos.x - chunk_pos.x).abs() > config.render_distance
            || (player_chunk_pos.z - chunk_pos.z).abs() > config.render_distance
        {
            commands.entity(chunk.id).insert(WantsToUnload(*chunk_pos));
        }
    }
}

pub fn mark_chunks_to_load(
    mut commands: Commands,
    config: Res<Config>,
    player_pos: Query<&Transform, With<FlyCam>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let player_chunk_pos = player_pos.single().translation / CHUNK_SIZE as f32;
    let player_chunk_pos = player_chunk_pos.floor().as_ivec3();

    // Iterate over all chunk positions within render distance
    for x in (player_chunk_pos.x - config.render_distance)
        ..=(player_chunk_pos.x + config.render_distance)
    {
        for z in (player_chunk_pos.z - config.render_distance)
            ..=(player_chunk_pos.z + config.render_distance)
        {
            let chunk_pos = IVec3::new(x, 0, z);

            if chunk_manager.loaded.contains_key(&chunk_pos) {
                continue;
            }

            let id = commands.spawn(WantsToLoad(chunk_pos)).id();
            let voxels = generate_voxel_data(chunk_pos);
            let chunk = Chunk { id, voxels };
            chunk_manager.unloaded.insert(chunk_pos, chunk);
        }
    }
}

pub fn load_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wants_to_load: Query<&WantsToLoad>,
) {
    for WantsToLoad(pos) in wants_to_load.iter() {
        if let Some(chunk) = chunk_manager.unloaded.remove(pos) {
            let (mesh, _collider) = generate_mesh(pos, &chunk);

            commands.entity(chunk.id).insert(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::GREEN.into()),
                ..default()
            });

            chunk_manager.loaded.insert(*pos, chunk);
        }
    }
}

pub fn unload_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    wants_to_unload: Query<&WantsToUnload>,
) {
    for WantsToUnload(pos) in wants_to_unload.iter() {
        if let Some(chunk) = chunk_manager.loaded.remove(pos) {
            commands.entity(chunk.id).despawn();
        }
    }
}
