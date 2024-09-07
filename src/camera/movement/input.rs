use bevy::prelude::*;

use crate::{
    camera::{CameraMarker, CameraSpeed},
    input::PlayerInput,
    states::AppState,
};

pub struct InputMovementPlugin;

impl Plugin for InputMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input.run_if(in_state(AppState::Editing)));
    }
}

/// This System moves the camera according to user input
fn handle_input(
    camera_speed: Res<CameraSpeed>,
    player_input: Res<PlayerInput>,
    mut camera_transform: Query<&mut Transform, With<CameraMarker>>,
) {
    camera_transform.single_mut().translation +=
        player_input.camera_vector().extend(0.) * camera_speed.0;
}
