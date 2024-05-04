pub mod zoom;

use bevy::prelude::*;
use zoom::ZoomPlugin;
use crate::utilities::easing::TimeEase;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (Camera2dBundle::default(), CameraMarker, TimeEase::default());
        app.world.spawn(bundle);
        app.add_plugins(ZoomPlugin);
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
struct CameraMarker;
