pub mod zoom;
use crate::utilities::{movement::{CameraMovementPlugin, FollowMarker}, easing::TimeEase};
use bevy::prelude::*;
use zoom::ZoomPlugin;

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
        app.add_plugins((ZoomPlugin, CameraMovementPlugin));
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
pub struct CameraMarker;
