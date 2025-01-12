use bevy::prelude::*;
use bevy_tnua::prelude::TnuaBuiltinWalk;

use crate::player::{JumpHeight, PlayerBundle, Speed};

/// The Plugin that allows for configuration
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterControllerSettings::default())
            .insert_resource(PlayerSettings::default())
            .insert_resource(LevelSettings::default())
            .insert_resource(CameraSettings::default());
    }
}

/// The Resource that configures the Character Controller
#[derive(Resource)]
pub struct CharacterControllerSettings {
    pub builtin_walk: TnuaBuiltinWalk,
}

impl Default for CharacterControllerSettings {
    fn default() -> Self {
        CharacterControllerSettings {
            builtin_walk: TnuaBuiltinWalk {
                float_height: 0.5,
                spring_strengh: 1600.,
                acceleration: 5000.,
                air_acceleration: 5000.,
                ..default()
            },
        }
    }
}

/// The Resource that configures the player
#[derive(Resource)]
pub struct PlayerSettings {
    pub player_bundle: PlayerBundle,
    pub sensor_collider_size: Vec2,
    pub collider_size: Vec2,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            player_bundle: PlayerBundle {
                jump_height: JumpHeight(40.),
                speed: Speed(200.),
                name: Name::new("Player"),
                ..default()
            },
            collider_size: Vec2::new(12., 14.),
            sensor_collider_size: Vec2::new(11.5, 13.5),
        }
    }
}

/// The Resource that configures the level
#[derive(Resource)]
pub struct LevelSettings {
    pub death_height: f32,
    pub spawn_location: Transform,
}

impl Default for LevelSettings {
    fn default() -> Self {
        LevelSettings {
            death_height: -100.,
            spawn_location: Transform::from_xyz(0., 30., 10.),
        }
    }
}

/// The Resource that configures the camera
#[derive(Resource)]
pub struct CameraSettings {
    pub default_zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings { default_zoom: 7. }
    }
}
