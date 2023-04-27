use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::fmt::Write;

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::WHITE;

pub struct ScreenDiagnosticsPlugin;

impl Plugin for ScreenDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(spawn_text)
            .add_system(update_fps_text)
            .add_system(update_pos_text);
    }
}

#[derive(Component)]
struct ScreenDiagnosticsText;

#[derive(Component)]
struct PlayerPositionText;

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    commands
        .spawn(
            TextBundle::from_sections([TextSection {
                value: "FPS: ...".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: FONT_SIZE,
                    color: FONT_COLOR,
                },
            }])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScreenDiagnosticsText);

    commands
        .spawn(
            TextBundle::from_section(
                "X: ... Y: ... Z: ...".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: FONT_SIZE,
                    color: FONT_COLOR,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(PlayerPositionText);
}

fn update_fps_text(
    diagnostics: Res<Diagnostics>,
    mut fps_text_query: Query<&mut Text, With<ScreenDiagnosticsText>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed());

    if let Ok(mut text) = fps_text_query.get_single_mut() {
        let value = &mut text.sections[0].value;
        value.clear();

        if let Some(fps) = fps {
            write!(value, "FPS: {:.0}", fps).unwrap();
        } else {
            write!(value, "FPS: ...").unwrap();
        }
    }
}

fn update_pos_text(
    mut pos_text_query: Query<&mut Text, With<PlayerPositionText>>,
    player_transform_query: Query<&Transform, With<FlyCam>>,
) {
    let pos = player_transform_query.get_single().unwrap().translation / 32.0;

    if let Ok(mut text) = pos_text_query.get_single_mut() {
        let value = &mut text.sections[0].value;
        value.clear();

        write!(value, "X: {:.2} Y: {:.2} Z: {:.2}", pos.x, pos.y, pos.z).unwrap();
    }
}
