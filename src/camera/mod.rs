pub mod zoom;
pub mod movement;
use bevy::prelude::*;
use zoom::ZoomPlugin;
use movement::{CameraMovementPlugin, FollowMarker, MovementMode};
use crate::utilities::easing::TimeEase;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (
            Camera2dBundle::default(),
            CameraMarker,
            TimeEase::default(),
            FollowMarker::new(0),
        );
        app.world.spawn(bundle);
        app.add_plugins((ZoomPlugin,CameraMovementPlugin));
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
pub struct CameraMarker;
