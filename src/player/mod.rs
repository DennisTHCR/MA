mod animation;
mod movement;

use crate::camera::movement::follow::{Target, TargetMarker};
use animation::PlayerAnimationPlugin;
use bevy::prelude::*;
use movement::PlayerMovementPlugin;

/// A Plugin containing everything related to the player
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerAnimationPlugin);
    }
}

/// A Bundle containing everything needed by the player entity
#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub jump_height: JumpHeight,
    pub speed: Speed,
    pub target_marker: TargetMarker,
    pub name: Name,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            marker: PlayerMarker,
            jump_height: JumpHeight(200.),
            speed: Speed(1000.),
            target_marker: TargetMarker::new(Target::Player),
            name: Name::new("Player"),
        }
    }
}

/// A Marker to identify the player
#[derive(Component, Clone, Copy)]
pub struct PlayerMarker;

/// The Component that defines the players jump height
#[derive(Component, Clone, Copy)]
pub struct JumpHeight(pub f32);

/// The Component that defines the players speed
#[derive(Component, Clone, Copy)]
pub struct Speed(pub f32);
