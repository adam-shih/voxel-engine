use bevy::prelude::*;
use bevy_flycam::prelude::*;
use voxel_engine::chunk::{Chunk, CHUNK_SIZE};
use voxel_engine::chunk_manager::*;
use voxel_engine::diagnostic::ScreenDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScreenDiagnosticsPlugin)
        .add_plugin(ChunkManagerPlugin)
        // .add_startup_system(spawn_single_chunk_at_origin)
        .add_startup_system(setup)
        .insert_resource(MovementSettings {
            speed: 6.0, // default: 12.0
            ..default()
        })
        .run();
}

pub fn spawn_single_chunk_at_origin(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk = Chunk::new(IVec3::ONE);
    let mesh = chunk.mesh_data.create_mesh();

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::GREEN.into()),
        ..default()
    });

    for (pos, voxel) in chunk.voxel_data.voxels.iter() {
        if !voxel.is_active {
            continue;
        }

        let chunk_offset = (chunk.position * CHUNK_SIZE).as_vec3();

        let mut sphere = shape::UVSphere::default();
        sphere.radius = 0.125;

        commands.spawn(PbrBundle {
            mesh: meshes.add(sphere.into()),
            material: materials.add(Color::SILVER.into()),
            transform: Transform::from_translation(pos.as_vec3() + chunk_offset),
            ..default()
        });
    }
}

fn setup(mut commands: Commands) {
    // Spawn a simple light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 40.0, 0.0),
        ..default()
    });
}
