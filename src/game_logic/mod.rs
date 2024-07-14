use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::player::PlayerMarker;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_player_position);
    }
}

fn check_player_position(mut query: Query<(&mut Transform, &mut Velocity), With<PlayerMarker>>) {
    let (mut transform, mut velocity) = query.single_mut();
    if transform.translation.y < -100. {
        transform.translation = Vec3::new(0., 30., 10.);
        velocity.linvel = Vec2::ZERO;
    } 
}