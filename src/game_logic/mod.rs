use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{config::PlayerSettings, player::PlayerMarker};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_player_position);
    }
}

fn check_player_position(ps: Res<PlayerSettings>, mut query: Query<(&mut Transform, &mut Velocity), With<PlayerMarker>>) {
    let (mut transform, mut velocity) = query.single_mut();
    if transform.translation.y < -100. {
        *transform = ps.spawn_location;
        velocity.linvel = Vec2::ZERO;
    }
}
