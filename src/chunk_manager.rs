use crate::voxel::{generate_mesh, generate_voxel_data, Chunk};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::HashMap;

const RENDER_DISTANCE: i32 = 1;

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
    player_pos: Query<&Transform, With<FlyCam>>,
    chunk_manager: ResMut<ChunkManager>,
) {
    let player_pos = player_pos.single().translation.as_ivec3();

    for (chunk_pos, chunk) in chunk_manager.loaded.iter() {
        let chunk_pos_vec2 = IVec2::new(chunk_pos.x, chunk_pos.z).as_vec2();
        let player_pos_vec2 = IVec2::new(player_pos.x, player_pos.z).as_vec2();
        if chunk_pos_vec2.distance(player_pos_vec2).abs() > RENDER_DISTANCE as f32 + 1.0 {
            commands.entity(chunk.id).insert(WantsToUnload(*chunk_pos));
        }
    }
}

pub fn mark_chunks_to_load(
    mut commands: Commands,
    player_pos: Query<&Transform, With<FlyCam>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let pos = player_pos.single().translation.as_ivec3();

    // Iterate over all chunk positions within render distance
    for x in (pos.x - RENDER_DISTANCE)..=(pos.x + RENDER_DISTANCE) {
        for z in (pos.z - RENDER_DISTANCE)..=(pos.z + RENDER_DISTANCE) {
            let chunk_pos = IVec3::new(x, 0, z);

            if chunk_manager.loaded.contains_key(&chunk_pos) {
                continue;
            }

            // If chunk is unloaded, mark as WantsToLoad
            if let Some(chunk) = chunk_manager.unloaded.get(&chunk_pos) {
                commands.entity(chunk.id).insert(WantsToLoad(chunk_pos));
            }
            // Otherwise generate chunk in unloaded and mark
            else {
                let id = commands.spawn(WantsToLoad(chunk_pos)).id();
                let voxels = generate_voxel_data(chunk_pos);
                let chunk = Chunk { id, voxels };
                chunk_manager.unloaded.insert(chunk_pos, chunk);
            }
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
