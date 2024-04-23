use bevy::prelude::*;
mod camera;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                camera::CameraPlugin,
            ));
    }
}