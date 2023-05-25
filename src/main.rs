use bevy::prelude::*;
use bevy_flycam::prelude::*;
use voxel_engine::chunk_manager::*;
use voxel_engine::diagnostic::ScreenDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ScreenDiagnosticsPlugin)
        .add_plugin(ChunkManagerPlugin)
        .add_system(setup.on_startup())
        .run();
}

fn setup(mut commands: Commands) {
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
