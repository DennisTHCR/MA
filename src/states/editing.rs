use std::path::Path;

use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::prelude::Collider;
use super::AppState;
use crate::input::PlayerInput;
use crate::utilities::assets::{Column, Row};
use crate::{
    camera::{movement::MovementMode, CameraMarker},
    config::{LevelSettings, PlayerSettings},
    utilities::assets::{ColorResource, ImageHandles, Material},
};
use crate::level_management::{LevelEntities, LevelMaterials};

pub struct EditingPlugin;

impl Plugin for EditingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Editing), enter_editing)
            .add_systems(OnExit(AppState::Editing), exit_editing)
            .add_systems(
                Update,
                (move_block_to_cursor, change_block_type, place_block).run_if(in_state(AppState::Editing)),
            );
    }
}

#[derive(Component)]
struct SpawnIndicatorMarker;

#[derive(Component)]
struct DeathLineMarker;

#[derive(Component)]
struct PlacingBlockMarker;

fn move_block_to_cursor(
    mut block_transform: Query<&mut Transform, With<PlacingBlockMarker>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let x = point.x - ((point.x % 16.) + 16.) % 16. + 8.;
    let y = point.y - ((point.y % 16.) + 16.) % 16. + 8.;
    let grid_point = Vec2::new(x, y);

    block_transform.single_mut().translation = grid_point.extend(10.);
}

fn place_block(
    input: Res<PlayerInput>,
    mut level_materials: ResMut<LevelMaterials>,
    mut level_entities: ResMut<LevelEntities>,
    query: Query<(&Transform, &Material), With<PlacingBlockMarker>>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
) {
    if !input.left_clicked() {
        return
    }
    let (transform, material) = query.single();
    let mut translation = transform.translation;
    translation.z = 0.;
    let position = ((translation.x as i32 - 8) / 16, (translation.y as i32 - 8) / 16);
    let (x,y) = position;
    println!("{:?}", position);
    level_materials.0.insert(position, Some(material.clone()));
    let row = level_materials.get_row(position.clone());
    let column = level_materials.get_column(position.clone());
    let entity = commands.spawn((
        SpriteBundle {
            texture: image_handles.0.get(&(material.clone(), row, column)).unwrap().clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
        Collider::cuboid(16. / 2., 16. / 2.),
        Name::new("Textured Block"),
    ));
    level_entities.0.insert(position.clone(), entity.id());
    let neighbours = [(x,y+1), (x,y-1), (x+1, y), (x-1, y)];
    neighbours.iter().for_each(|pos| {
        let id = level_entities.0.get(pos);
        if id.is_none() {
            return
        }
        let mut entity = commands.entity(id.unwrap().clone());
        let row = level_materials.get_row(pos.clone());
        let column = level_materials.get_column(pos.clone());
        let material = level_materials.0.get(pos).unwrap();
        if material.is_none() {
            entity.remove::<Handle<Image>>();
            return
        }
        let texture = image_handles.0.get(&(material.unwrap().clone(), row, column)).unwrap().clone();
        entity.insert(texture);
    })
}

fn change_block_type(
    mut query: Query<(&mut Handle<Image>, &mut Material), With<PlacingBlockMarker>>,
    input: Res<PlayerInput>,
    textures: Res<ImageHandles>,
) {
    if input.right_clicked() {
        query.iter_mut().for_each(|(mut image, mut material)| {
            *material = match *material {
                Material::GRASS_GREEN => Material::GRASS_ORANGE,
                Material::GRASS_ORANGE => Material::GRASS_PINK,
                Material::GRASS_PINK => Material::WOOD,
                Material::WOOD => Material::STEEL,
                Material::STEEL => Material::BRONZE,
                Material::BRONZE => Material::BRICK,
                Material::BRICK => Material::GOLD,
                Material::GOLD => Material::GRASS_GREEN,
            };
            *image = textures
                .0
                .get(&(*material, Row::TOP, Column::LEFT))
                .unwrap()
                .clone();
        })
    }
}

fn enter_editing(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut MovementMode, With<CameraMarker>>,
    mut commands: Commands,
    ls: Res<LevelSettings>,
    materials: Res<ColorResource>,
    ps: Res<PlayerSettings>,
    textures: Res<ImageHandles>,
    asset_server: Res<AssetServer>,
) {
    query.iter_mut().for_each(|mut movement_mode| {
        *movement_mode = MovementMode::Input;
    });
    let character_handle = asset_server.load(Path::new("CHARACTER.png"));
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
    let block_handle = textures
        .0
        .get(&(Material::GRASS_GREEN, Row::TOP, Column::LEFT))
        .unwrap()
        .clone();
    commands.spawn((
        SpriteBundle {
            texture: block_handle,
            ..default()
        },
        Material::GRASS_GREEN,
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
