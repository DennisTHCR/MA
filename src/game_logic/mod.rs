use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{config::LevelSettings, player::PlayerMarker, states::AppState};

/// The Plugin containing all game logic
pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_player_position.run_if(in_state(AppState::Playing)),
        );
    }
}

/// A System that kills the player if he falls
fn check_player_position(
    ls: Res<LevelSettings>,
    mut query: Query<(&mut Transform, &mut Velocity), With<PlayerMarker>>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    if transform.translation.y < ls.death_height {
        *transform = ls.spawn_location;
        velocity.linvel = Vec2::ZERO;
    }
}
