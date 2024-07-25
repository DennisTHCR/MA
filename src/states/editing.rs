use std::path::Path;

use bevy::prelude::*;

use crate::{camera::{movement::MovementMode, CameraMarker}, config::LevelSettings};

use super::AppState;

pub struct EditingPlugin;

impl Plugin for EditingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Editing), enter_editing)
           .add_systems(OnExit(AppState::Editing), exit_editing);
    }
}

#[derive(Component)]
struct SpawnIndicatorMarker;

fn enter_editing(mut query: Query<&mut MovementMode, With<CameraMarker>>, mut commands: Commands, ls: Res<LevelSettings>, asset_server: Res<AssetServer>) {
    query.iter_mut().for_each(|mut movement_mode| {
        *movement_mode = MovementMode::Input;
    });
    let handle = asset_server.load(Path::new("test.png"));
    commands.spawn((
        SpriteBundle {
            texture: handle,
            transform: ls.spawn_location,
            ..default()
        },
        SpawnIndicatorMarker,
    ));
}

fn exit_editing(spawn_indicator: Query<Entity, With<SpawnIndicatorMarker>>, mut commands: Commands) {
    commands.entity(spawn_indicator.single()).despawn();
}
