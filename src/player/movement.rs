use super::{JumpHeight, Speed};
use crate::{config::CharacterControllerSettings, input::PlayerInput, states::AppState};
use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinCrouch, prelude::*};
use bevy_tnua_rapier2d::*;

/// The Plugin that implements movement for the player
pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement.run_if(in_state(AppState::Playing)))
            .add_plugins((
                TnuaRapier2dPlugin::default(),
                TnuaControllerPlugin::default(),
            ));
    }
}

/// A system that moves the player according to [bevy_tnua] and [bevy_tnua_rapier2d]
fn apply_movement(
    ccs: Res<CharacterControllerSettings>,
    mut query: Query<(&mut TnuaController, &Speed, &JumpHeight, &mut Sprite)>,
    input: Res<PlayerInput>,
) {
    query
        .iter_mut()
        .for_each(|(mut controller, speed, jump_height, mut sprite)| {
            controller.basis(TnuaBuiltinWalk {
                desired_velocity: input.direction_vector().extend(0.) * speed.0,
                desired_forward: input.direction_vector().extend(0.),
                ..ccs.builtin_walk
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

            let x = input.direction_vector().x;

            if x > 0. {
                sprite.flip_x = false
            } else if x < 0. {
                sprite.flip_x = true
            }
        })
}
