pub mod zoom;
use crate::utilities::{easing::{TimeEase, EasingFunction, EasingType}, movement::{{CameraMovementPlugin, MovementMode}, follow::FollowMarker}};
use bevy::prelude::*;
use zoom::ZoomPlugin;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (
            Camera2dBundle::default(),
            CameraMarker,
            TimeEase::new(
                0,
                1000,
                0.,
                10.,
                EasingFunction::Sine,
                EasingType::Out
            ),
            FollowMarker::new(0),
            MovementMode::Follow,
        );
        app.world.spawn(bundle);
        app.add_plugins((ZoomPlugin, CameraMovementPlugin));
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
pub struct CameraMarker;
