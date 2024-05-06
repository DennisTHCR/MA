pub mod zoom;
mod movement;
use bevy::prelude::*;
use zoom::{ZoomPlugin, ZoomEase};

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (
            Camera2dBundle::default(),
            CameraMarker,
            ZoomEase::default(),
        );
        app.world.spawn(bundle);
        app.add_plugins((ZoomPlugin,));
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
struct CameraMarker;
