use crate::{
    physics::{Gravity, GravityForce, InputOffset, JumpForce, Offset},
    utilities::movement::follow::TargetMarker,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            transform: Transform::from_xyz(0., 30., 10.),
            ..default()
        },
        PlayerMarker,
        TargetMarker::new(0),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(16., 16.),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(1.),
            snap_to_ground: Some(CharacterLength::Absolute(1.2)),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
        Gravity(9.0),
        GravityForce(Vec2::ZERO),
        Offset(Vec2::ZERO),
        InputOffset(Vec2::ZERO),
        JumpForce(Vec2::ZERO),
        JumpHeight(5.),
    ));
}

#[derive(Component)]
pub struct PlayerMarker;
#[derive(Component)]
pub struct JumpHeight(pub f32);
#[derive(Resource)]
pub struct PlayerSpeed(pub f32);
