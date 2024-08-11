use std::path::Path;

use super::AppState;
use crate::input::PlayerInput;
use crate::level_management::Level;
use crate::utilities::assets::{Column, Row};
use crate::{
    camera::{movement::MovementMode, CameraMarker},
    config::{LevelSettings, PlayerSettings},
    utilities::assets::{ColorResource, ImageHandles, Material},
};
use bevy::{prelude::*, sprite::Mesh2dHandle};

pub struct EditingPlugin;

impl Plugin for EditingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Editing), enter_editing)
            .add_systems(OnExit(AppState::Editing), exit_editing)
            .add_systems(
                Update,
                (move_block_to_cursor, change_block_type, place_block)
                    .run_if(in_state(AppState::Editing)),
            )
            .insert_resource(HoveringBlock::default());
    }
}

#[derive(Component)]
struct SpawnIndicatorMarker;

#[derive(Component)]
struct DeathLineMarker;

#[derive(Resource)]
struct HoveringBlock {
    last_hovered: (i32, i32),
    hovering: (i32, i32),
    selected_material: Option<Material>,
    original_material: Option<Material>,
}

impl Default for HoveringBlock {
    fn default() -> Self {
        HoveringBlock {
            last_hovered: (0,0),
            hovering: (0,0),
            selected_material: None,
            original_material: None,
        }
    }
}

impl HoveringBlock {
    fn new_cursor_position(&mut self, (x,y): (i32,i32)) {
        if self.hovering != (x,y) {
            self.hovering = (x,y);
        }
    }
}

fn move_block_to_cursor(
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
    let grid_point = (x, y);
    
}

fn place_block(
    input: Res<PlayerInput>,
    mut level: ResMut<Level>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    hovering_block: Res<HoveringBlock>,
) {
    if !input.left_clicked() {
        return;
    }
    let position = hovering_block.hovering;
    let material = hovering_block.selected_material;
    level.insert(position, material.clone(), &mut commands, &image_handles);
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
}

fn exit_editing(
    spawn_indicator: Query<Entity, With<SpawnIndicatorMarker>>,
    death_marker: Query<Entity, With<DeathLineMarker>>,
    mut commands: Commands,
) {
    commands.entity(spawn_indicator.single()).despawn();
    commands.entity(death_marker.single()).despawn();
}
