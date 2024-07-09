use bevy::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{
    physics::{InputOffset, JumpForce},
    player::{JumpHeight, PlayerMarker, PlayerSpeed},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut JumpForce,
            &mut InputOffset,
            &KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &JumpHeight,
        ),
        With<PlayerMarker>,
    >,
    speed: Res<PlayerSpeed>,
    time: Res<Time>,
) {
    let (
        mut jump_force,
        mut input_offset,
        character_controller,
        character_controller_output,
        jump_height,
    ) = query.single_mut();
    let movement = speed.0 * time.delta_seconds();
    input_offset.0 = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowRight) {
        input_offset.0.x += movement;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        input_offset.0.x -= movement;
    }
    if input.just_pressed(KeyCode::ArrowUp) && character_controller_output.grounded {
        jump_force.0 += character_controller.up * jump_height.0;
    }
}
