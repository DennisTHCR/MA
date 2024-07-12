use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinCrouch, prelude::*};
use bevy_tnua_rapier2d::*;
use crate::input::PlayerInput;
use super::{JumpHeight, Speed};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_movement)
            .add_plugins((TnuaRapier2dPlugin::default(), TnuaControllerPlugin::default()));
    }
}

fn apply_movement(mut query: Query<(&mut TnuaController, &Speed, &JumpHeight)>, input: Res<PlayerInput>) {
    query.iter_mut().for_each(|(mut controller, speed, jump_height)| {
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: input.direction_vector().extend(0.) * speed.0,
            desired_forward: input.direction_vector().extend(0.),
            float_height: 2.5,
            spring_strengh: 1200.,
            acceleration: 500.,
            air_acceleration: 500.,
            ..default()
        });

        if input.jump_pressed() {
            controller.action(TnuaBuiltinJump {
                height: jump_height.0,
                ..default()
            });
        }

        if input.crouch_pressed() {
            controller.action(TnuaBuiltinCrouch::default());
        }
    })
}