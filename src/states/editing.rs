use std::path::Path;

use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    camera::{movement::MovementMode, CameraMarker},
    config::{LevelSettings, PlayerSettings},
    utilities::assets::ColorResource,
};

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

#[derive(Component)]
struct DeathLineMarker;

fn enter_editing(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut MovementMode, With<CameraMarker>>,
    mut commands: Commands,
    ls: Res<LevelSettings>,
    asset_server: Res<AssetServer>,
    materials: Res<ColorResource>,
    ps: Res<PlayerSettings>,
) {
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
    let mesh: Mesh2dHandle = meshes.add(Rectangle::new(10000000., 10.)).into();
    commands.spawn((
        ColorMesh2dBundle {
            mesh,
            material: materials.0[1].0.clone(),
            transform: Transform::from_xyz(0., ls.death_height - ps.collider_size.y, -10.),
            ..default()
        },
        DeathLineMarker,
    ));
}

fn exit_editing(
    spawn_indicator: Query<Entity, With<SpawnIndicatorMarker>>,
    death_marker: Query<Entity, With<DeathLineMarker>>,
    mut commands: Commands,
) {
    commands.entity(spawn_indicator.single()).despawn();
    commands.entity(death_marker.single()).despawn();
}
