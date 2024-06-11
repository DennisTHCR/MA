use crate::utilities::movement::follow::TargetMarker;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .insert_resource(PlayerSpeed(30.));
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("test.png");
    commands.spawn((
        SpriteBundle {
            texture: handle,
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        PlayerMarker,
        TargetMarker::new(0),
    ));
}

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Resource)]
pub struct PlayerSpeed(f32);
