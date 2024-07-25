use std::path::Path;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody};
use bevy_tnua::prelude::TnuaControllerBundle;
use bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape};

use crate::{
    camera::{
        movement::follow::{FollowMarker, Target},
        CameraMarker,
    },
    config::{LevelSettings, PlayerSettings},
    player::{PlayerBundle, PlayerMarker},
};

use super::AppState;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Playing), exit_playing)
            .add_systems(OnEnter(AppState::Playing), enter_playing);
    }
}

fn exit_playing(
    mut commands: Commands,
    cameras: Query<Entity, With<CameraMarker>>,
    player: Query<Entity, With<PlayerMarker>>,
) {
    cameras.iter().for_each(|entity| {
        commands.entity(entity).remove::<FollowMarker>();
    });
    commands.entity(player.single()).despawn();
}

fn enter_playing(
    mut commands: Commands,
    cameras: Query<Entity, With<CameraMarker>>,
    asset_server: Res<AssetServer>,
    ls: Res<LevelSettings>,
    ps: Res<PlayerSettings>,
) {
    let handle = asset_server.load(Path::new("test.png"));
    commands.spawn((
        SpriteBundle {
            texture: handle,
            transform: ls.spawn_location,
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
        TnuaRapier2dSensorShape(Collider::cuboid(
            ps.sensor_collider_size.x,
            ps.sensor_collider_size.y,
        )),
        Collider::cuboid(ps.collider_size.x, ps.collider_size.y),
    ));
    cameras.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(FollowMarker::new(Target::Player));
    })
}
