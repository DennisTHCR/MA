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
    utilities::assets::{PlayerAnimationMap, PlayerAnimationState},
};

use super::AppState;

/// The Plugin containing everything related to [AppState::Playing]
pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Playing), exit_playing)
            .add_systems(OnEnter(AppState::Playing), enter_playing);
    }
}

/// This System is run whenever the [AppState] changes from [AppState::Playing] and cleans up everything unnescessary
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

/// This System is run whenever the [AppState] changes to [AppState::Playing] and prepares everything required
fn enter_playing(
    mut commands: Commands,
    cameras: Query<Entity, With<CameraMarker>>,
    ls: Res<LevelSettings>,
    ps: Res<PlayerSettings>,
    player_animation_map: Res<PlayerAnimationMap>,
) {
    let texture = player_animation_map
        .0
        .get(&PlayerAnimationState::IDLE)
        .unwrap()
        .texture
        .clone();
    commands.spawn((
        SpriteBundle {
            texture,
            transform: ls.spawn_location,
            ..default()
        },
        PlayerBundle {
            // Hack because Name doesn't implement clone and copy
            name: ps.player_bundle.name.clone(),
            ..ps.player_bundle
        },
        TextureAtlas {
            layout: player_animation_map
                .0
                .get(&PlayerAnimationState::IDLE)
                .unwrap()
                .texture_atlas_layout
                .clone(),
            index: 1,
        },
        RigidBody::Dynamic,
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        LockedAxes::ROTATION_LOCKED,
        TnuaRapier2dSensorShape(Collider::capsule(Vec2::new(0., -ps.sensor_collider_size.y / 2. - 2.), Vec2::new(0., ps.sensor_collider_size.y / 2. - 2.), ps.sensor_collider_size.x / 2.)),
        Collider::capsule(Vec2::new(0., -ps.collider_size.y / 2. - 2.), Vec2::new(0., ps.collider_size.y / 2. - 2.), ps.collider_size.x / 2.),
    ));
    cameras.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(FollowMarker::new(Target::Player));
    })
}
