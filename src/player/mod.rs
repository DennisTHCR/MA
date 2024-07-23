mod movement;

use std::path::Path;

use crate::camera::movement::follow::{Target, TargetMarker};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape};
use movement::PlayerMovementPlugin;
use crate::config::PlayerSettings;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_plugins(PlayerMovementPlugin);
    }
}

fn setup_player(ps: Res<PlayerSettings>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load(Path::new("test.png"));
    commands.spawn((
        SpriteBundle {
            texture: handle,
            transform: ps.spawn_location,
            ..default()
        },
        PlayerBundle {
            // Hack because Name doesn't implement clone and copy
            name: ps.player_bundle.name.clone(),
            ..ps.player_bundle
        },
        RigidBody::Dynamic,
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        LockedAxes::ROTATION_LOCKED,
        TnuaRapier2dSensorShape(Collider::cuboid(ps.sensor_collider_size.x, ps.sensor_collider_size.y)),
        Collider::cuboid(ps.collider_size.x, ps.collider_size.y),
    ));
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub jump_height: JumpHeight,
    pub speed: Speed,
    pub target_marker: TargetMarker,
    pub name: Name,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            marker: PlayerMarker,
            jump_height: JumpHeight(200.),
            speed: Speed(1000.),
            target_marker: TargetMarker::new(Target::Player),
            name: Name::new("Player"),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct PlayerMarker;
#[derive(Component, Clone, Copy)]
pub struct JumpHeight(pub f32);
#[derive(Component, Clone, Copy)]
pub struct Speed(pub f32);
