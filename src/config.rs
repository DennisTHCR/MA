use bevy::prelude::*;
use bevy_tnua::prelude::TnuaBuiltinWalk;

use crate::player::{JumpHeight, PlayerBundle, Speed};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterControllerSettings::default())
            .insert_resource(PlayerSettings::default())
            .insert_resource(LevelSettings::default());
    }
}

#[derive(Resource)]
pub struct CharacterControllerSettings {
    pub builtin_walk: TnuaBuiltinWalk,
}

impl Default for CharacterControllerSettings {
    fn default() -> Self {
        CharacterControllerSettings {
            builtin_walk: TnuaBuiltinWalk {
                float_height: 5.,
                spring_strengh: 1200.,
                acceleration: 5000.,
                air_acceleration: 5000.,
                ..default()
            },
        }
    }
}

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
                jump_height: JumpHeight(200.),
                speed: Speed(1000.),
                name: Name::new("Player"),
                ..default()
            },
            collider_size: Vec2::new(120., 140.),
            sensor_collider_size: Vec2::new(115., 135.),
        }
    }
}

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
