pub mod movement;
pub mod zoom;
use crate::config::CameraSettings;
use crate::utilities::easing::{EasingFunction, EasingType, TimeEase};
use bevy::prelude::*;
use movement::{
    follow::{FollowMarker, Target},
    {CameraMovementPlugin, MovementMode},
};
use zoom::ZoomPlugin;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (
            Camera2dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.54, 0.86, 0.92)),
                    ..default()
                },
                ..default()
            },
            CameraMarker,
            TimeEase::new(0, 1000, 0., 1., EasingFunction::Sine, EasingType::Out),
            FollowMarker::new(Target::Player),
            MovementMode::Follow,
        );
        app.add_plugins((ZoomPlugin, CameraMovementPlugin))
            .insert_resource(CameraSpeed(5.))
            .add_systems(Startup, setup)
            .world_mut()
            .spawn(bundle);
    }
}

fn setup(
    camera_settings: Res<CameraSettings>,
    mut time_ease: Query<&mut TimeEase, With<CameraMarker>>,
) {
    time_ease
        .single_mut()
        .set_end_val(camera_settings.default_zoom);
}

// Structs

/// Marks the main camera.
#[derive(Component)]
pub struct CameraMarker;

#[derive(Resource)]
pub struct CameraSpeed(pub f32);
