use bevy::prelude::*;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashMap;
use voxel_engine::voxel::{generate_mesh, generate_voxel_data, Chunk};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk_map = HashMap::new();
    let chunk_pos = IVec3::ZERO;
    chunk_map.insert(
        chunk_pos,
        Chunk {
            voxels: generate_voxel_data(chunk_pos),
        },
    );

    let (mesh, collider) = generate_mesh(&chunk_map);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(collider);

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
