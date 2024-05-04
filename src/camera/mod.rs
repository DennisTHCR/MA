pub mod zoom;

use bevy::prelude::*;
use zoom::ZoomPlugin;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (Camera2dBundle::default(), CameraMarker);
        app.world.spawn(bundle);
        app.add_plugins(ZoomPlugin);
    }
}

// Components and Resources

/// Marks the main camera.
#[derive(Component)]
struct CameraMarker;
