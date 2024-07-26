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
            .add_systems(OnExit(AppState::Editing), exit_editing)
            .add_systems(Update, move_block_to_cursor.run_if(in_state(AppState::Editing)));
    }
}

#[derive(Component)]
struct SpawnIndicatorMarker;

#[derive(Component)]
struct DeathLineMarker;

#[derive(Component)]
struct PlacingBlockMarker;

fn move_block_to_cursor(mut block_transform: Query<&mut Transform, With<PlacingBlockMarker>>, camera_query: Query<(&Camera, &GlobalTransform)>, windows: Query<&Window>) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    // TODO: FIX THIS IDK?
    let mut x;
    if point.x % 160. <= 80. {
        x = point.x - (point.x % 160.);
    } else {
        x = point.x + (160. - point.x % 160.);
    }
    let mut y;
    if point.y % 160. <= 80. {
        y = point.y - (point.y % 160.);
    } else {
        y = point.y + (160. - point.y % 160.);
    }
    let grid_point = Vec2::new(x,y);

    block_transform.single_mut().translation = grid_point.extend(10.);
}

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
    let character_handle = asset_server.load(Path::new("test.png"));
    commands.spawn((
        SpriteBundle {
            texture: character_handle,
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
    let block_handle = asset_server.load(Path::new("floor_middle.png"));
    commands.spawn((
        SpriteBundle {
            texture: block_handle,
            ..default()
        },
        PlacingBlockMarker,
    ));
}

fn exit_editing(
    spawn_indicator: Query<Entity, With<SpawnIndicatorMarker>>,
    death_marker: Query<Entity, With<DeathLineMarker>>,
    placing_block_marker: Query<Entity, With<PlacingBlockMarker>>,
    mut commands: Commands,
) {
    commands.entity(spawn_indicator.single()).despawn();
    commands.entity(death_marker.single()).despawn();
    commands.entity(placing_block_marker.single()).despawn();
}
