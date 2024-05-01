pub mod camera;
pub mod resources;

use bevy::prelude::*;
use camera::CameraPlugin;
use resources::ResourcesPlugin;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, ResourcesPlugin));
    }
}
