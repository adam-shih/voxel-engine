use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::fmt::Write;

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::BLACK;

pub struct ScreenDiagnosticsPlugin;

impl Plugin for ScreenDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(spawn_text)
            .add_system(update_text);
    }
}

#[derive(Component)]
pub struct ScreenDiagnosticsText;

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "FPS: ...".to_string(),
                    style: TextStyle {
                        font,
                        font_size: FONT_SIZE,
                        color: FONT_COLOR,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(ScreenDiagnosticsText);
}

fn update_text(
    diagnostics: Res<Diagnostics>,
    mut text_query: Query<&mut Text, With<ScreenDiagnosticsText>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average());

    if let Ok(mut text) = text_query.get_single_mut() {
        let value = &mut text.sections[0].value;
        value.clear();

        if let Some(fps) = fps {
            write!(value, "FPS: {:.0}", fps).unwrap();
        } else {
            write!(value, "FPS: ...").unwrap();
        }
    }
}
