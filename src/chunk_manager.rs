use crate::voxel::{generate_mesh, generate_voxel_data, Chunk};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::HashMap;

const RENDER_DISTANCE: i32 = 0;

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
pub struct WantsToUnload;

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
                println!("Wants to load: {:?}", chunk_pos);
                commands.entity(chunk.id).insert(WantsToLoad(chunk_pos));
            }
            // Otherwise generate chunk in unloaded and mark
            else {
                println!("Wants to load (and generate): {:?}", chunk_pos);
                let id = commands.spawn(WantsToLoad(chunk_pos)).id();
                let voxels = generate_voxel_data(chunk_pos);
                let chunk = Chunk { id, voxels };
                chunk_manager.unloaded.insert(chunk_pos, chunk);
            }
        }
    }
}

pub fn load_chunks_test(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wants_to_load: Query<&WantsToLoad>,
) {
    for WantsToLoad(pos) in wants_to_load.iter() {
        if let Some(chunk) = chunk_manager.unloaded.remove(pos) {
            let (mesh, _collider) = generate_mesh(pos, &chunk);

            commands.spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::GREEN.into()),
                ..default()
            });

            chunk_manager.loaded.insert(*pos, chunk);
        }
    }
}
