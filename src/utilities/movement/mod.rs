pub mod free;
pub mod follow;
use bevy::prelude::*;
use follow::following_movement_system;
use free::{update_time_ease, free_movement_system};

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
        );
    }
}

/// Enum to decide the movement mode.
#[derive(Component, Default)]
pub enum MovementMode {
    Follow,
    #[default]
    Free,
}

