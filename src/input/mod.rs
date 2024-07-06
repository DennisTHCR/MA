use bevy::prelude::*;

use crate::player::{PlayerMarker, PlayerSpeed};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    speed: Res<PlayerSpeed>,
    time: Res<Time>,
) {
    let movement = speed.0 * time.delta_seconds();
    println!("{:?}", movement);
    if input.pressed(KeyCode::ArrowRight) {
        transform.single_mut().translation.x += movement;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.single_mut().translation.x -= movement;
    }
    if input.pressed(KeyCode::ArrowUp) {
        transform.single_mut().translation.y += movement*2.;
    }
}
