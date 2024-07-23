pub mod zoom;
pub mod movement;
use crate::utilities::easing::{EasingFunction, EasingType, TimeEase};
use movement::{
    follow::{FollowMarker, Target},
    {CameraMovementPlugin, MovementMode},};
use bevy::prelude::*;
use zoom::ZoomPlugin;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (
            Camera2dBundle::default(),
            CameraMarker,
            TimeEase::new(0, 1000, 0., 1., EasingFunction::Sine, EasingType::Out),
            FollowMarker::new(Target::Player),
            MovementMode::Follow,
        );
        app.world_mut().spawn(bundle);
        app.add_plugins((ZoomPlugin, CameraMovementPlugin));
        app.insert_resource(CameraSpeed(50.));
    }
}

// Structs

/// Marks the main camera.
#[derive(Component)]
pub struct CameraMarker;

#[derive(Resource)]
pub struct CameraSpeed(pub f32);
