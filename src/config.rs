use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Config {
    pub render_distance: i32,
}
