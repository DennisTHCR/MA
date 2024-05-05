pub mod zoom;
mod movement;

use crate::utilities::{easing::TimeEase, movement::MovementBundle};
use bevy::prelude::*;
use zoom::ZoomPlugin;
use movement::CameraMovementPlugin;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (
            Camera2dBundle::default(),
            CameraMarker,
            TimeEase::default(),
            MovementBundle::new(100., 100., 1000.),
        );
        app.world.spawn(bundle);
        app.add_plugins((ZoomPlugin, CameraMovementPlugin));
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
struct CameraMarker;
