pub mod follow;
pub mod free;
pub mod input;
use bevy::prelude::*;
use follow::following_movement_system;
use free::{free_movement_system, update_time_ease};
use input::InputMovementPlugin;

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_time_ease,
                (following_movement_system, free_movement_system),
            )
                .chain(),
        )
        .add_plugins(InputMovementPlugin);
    }
}

/// Enum to decide the movement mode
#[allow(dead_code)]
#[derive(Component, Default)]
pub enum MovementMode {
    #[default]
    Follow,
    Free,
    Input,
}
