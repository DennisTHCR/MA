use bevy::prelude::*;
use crate::utilities::movement::*;
use super::CameraMarker;
use crate::PlayerMarker;

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, accelerate_towards_player);
    }
}

fn accelerate_towards_player(mut camera: Query<(&mut CurrentAcceleration, &Transform),With<CameraMarker>>, player: Query<&Transform, With<PlayerMarker>>, time: Res<Time>) {
    let (mut camera_acceleration, camera_transform) = camera.single_mut();
    let player_transform = player.single();
    let delta_pos = player_transform.translation - camera_transform.translation;
    camera_acceleration.0 += Vec2::new(delta_pos.x, delta_pos.y) * time.delta_seconds();
}
