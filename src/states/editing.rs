use std::path::Path;

use super::AppState;
use crate::input::PlayerInput;
use crate::level_management::Level;
use crate::{
    camera::{movement::MovementMode, CameraMarker},
    config::{LevelSettings, PlayerSettings},
    utilities::assets::{ColorResource, Material},
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
            last_hovered: (0, 0),
            hovering: (0, 0),
            selected_material: None,
            original_material: None,
        }
    }
}

impl HoveringBlock {
    fn new_cursor_position(&mut self, (x, y): (i32, i32)) {
        if self.hovering != (x, y) {
            self.hovering = (x, y);
        }
    }
}

fn move_block_to_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut hovering_block: ResMut<HoveringBlock>,
    mut level: ResMut<Level>,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let x = (point.x as f32 / 16.0).floor() as i32;
    let y = (point.y as f32 / 16.0).floor() as i32;

    let grid_point = (x, y);
    hovering_block.new_cursor_position(grid_point);
    if hovering_block.hovering != hovering_block.last_hovered {
        level.insert(
            hovering_block.last_hovered,
            hovering_block.original_material,
        );
        let hovering_material = level.material_map.get(&grid_point);
        hovering_block.original_material = if hovering_material.is_some() {
            Some(*hovering_material.unwrap())
        } else {
            None
        };
        hovering_block.last_hovered = hovering_block.hovering;
    }
    level.insert(hovering_block.hovering, hovering_block.selected_material);
}

fn place_block(
    input: Res<PlayerInput>,
    mut level: ResMut<Level>,
    mut hovering_block: ResMut<HoveringBlock>,
) {
    if !input.left_clicked() {
        return;
    }
    let position = hovering_block.hovering;
    if hovering_block.selected_material.is_some() {
        let material = hovering_block.selected_material.unwrap();
        level.insert(position, Some(material.clone()));
    }
    hovering_block.original_material = hovering_block.selected_material;
}

fn change_block_type(mut hovering_block: ResMut<HoveringBlock>, input: Res<PlayerInput>) {
    if input.right_clicked() {
        let material = &mut hovering_block.selected_material;
        *material = match *material {
            Some(Material::GRASS_GREEN) => Some(Material::GRASS_ORANGE),
            Some(Material::GRASS_ORANGE) => Some(Material::GRASS_PINK),
            Some(Material::GRASS_PINK) => Some(Material::WOOD),
            Some(Material::WOOD) => Some(Material::STEEL),
            Some(Material::STEEL) => Some(Material::BRONZE),
            Some(Material::BRONZE) => Some(Material::BRICK),
            Some(Material::BRICK) => Some(Material::GOLD),
            Some(Material::GOLD) => None,
            None => Some(Material::GRASS_GREEN),
        };
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
