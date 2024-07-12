mod movement;

use crate::utilities::movement::follow::TargetMarker;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape};
use movement::PlayerMovementPlugin;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_plugins(PlayerMovementPlugin);
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
        PlayerBundle::default(),
        RigidBody::Dynamic,
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        LockedAxes::ROTATION_LOCKED,
        TnuaRapier2dSensorShape(Collider::cuboid(15.5, 15.5)),
        Collider::cuboid(16., 16.),
    ));
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    jump_height: JumpHeight,
    speed: Speed,
    target_marker: TargetMarker,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            marker: PlayerMarker,
            jump_height: JumpHeight(50.),
            speed: Speed(200.),
            target_marker: TargetMarker::new(0),
        }
    }
}

#[derive(Component)]
pub struct PlayerMarker;
#[derive(Component)]
pub struct JumpHeight(pub f32);
#[derive(Component)]
pub struct Speed(pub f32);
