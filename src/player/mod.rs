mod movement;

use crate::camera::movement::follow::{Target, TargetMarker};
use bevy::prelude::*;
use movement::PlayerMovementPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin);
    }
}

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

#[derive(Component, Clone, Copy)]
pub struct PlayerMarker;
#[derive(Component, Clone, Copy)]
pub struct JumpHeight(pub f32);
#[derive(Component, Clone, Copy)]
pub struct Speed(pub f32);
