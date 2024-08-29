use std::path::Path;

use super::AppState;
use crate::input::PlayerInput;
use crate::level_management::{Level,execute_level_queues};
use crate::{
    camera::{movement::MovementMode, CameraMarker},
    config::{LevelSettings, PlayerSettings},
    utilities::assets::{ColorResource, Material},
};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use std::fs::write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct EditingPlugin;

impl Plugin for EditingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Editing), enter_editing)
            .add_systems(OnExit(AppState::Editing), exit_editing)
            .add_systems(
                Update,
                (move_block_to_cursor, change_block_type, place_block).before(execute_level_queues)
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
pub struct HoveringBlock {
    pub last_hovered: (i32, i32),
    pub hovering: (i32, i32),
    pub selected_material: Option<Material>,
    pub original_material: Option<Material>,
}

impl Default for HoveringBlock {
    fn default() -> Self {
        HoveringBlock {
            last_hovered: (0, 0),
            hovering: (0, 0),
            selected_material: Some(Material::GRASS_GREEN),
            original_material: None,
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
    
    // Check if the cursor has moved to a new block.
    if grid_point != hovering_block.hovering {
        // Restore the original material of the last hovered block.
        level.insert(hovering_block.hovering, hovering_block.original_material);
        
        // Update the `last_hovered` to the new hovered block.
        hovering_block.last_hovered = hovering_block.hovering;
        
        // Update the `hovering` position to the new block.
        hovering_block.hovering = grid_point;
        
        // Fetch and store the original material of the new block.
        hovering_block.original_material = level.material_map.get(&grid_point).copied();
        
        // Temporarily set the new hovered block's material to the selected material (which could be `None`).
        level.insert(grid_point, hovering_block.selected_material);
    }
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

fn change_block_type(mut hovering_block: ResMut<HoveringBlock>, input: Res<PlayerInput>, mut level: ResMut<Level>) {
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
        level.insert(hovering_block.hovering, hovering_block.selected_material);
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

mod tuple_map {
    use super::*;
    use serde::{Deserializer, Serializer};
    use std::collections::HashMap;

    pub fn serialize<S>(
        map: &HashMap<(i32, i32), Material>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_map: HashMap<String, &Material> = map
            .iter()
            .map(|(k, v)| (format!("{},{}", k.0, k.1), v))
            .collect();
        string_map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<(i32, i32), Material>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_map: HashMap<String, Material> = HashMap::deserialize(deserializer)?;
        let map = string_map
            .into_iter()
            .map(|(k, v)| {
                let mut split = k.split(',');
                let x = split
                    .next()
                    .and_then(|x| x.parse::<i32>().ok())
                    .ok_or_else(|| serde::de::Error::custom("Invalid key format"))?;
                let y = split
                    .next()
                    .and_then(|y| y.parse::<i32>().ok())
                    .ok_or_else(|| serde::de::Error::custom("Invalid key format"))?;
                Ok(((x, y), v))
            })
            .collect::<Result<HashMap<(i32, i32), Material>, _>>()?;
        Ok(map)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerdeMapContainer {
    #[serde(with="tuple_map")]
    pub map: HashMap<(i32,i32), Material>
}

fn exit_editing(
    spawn_indicator: Query<Entity, With<SpawnIndicatorMarker>>,
    death_marker: Query<Entity, With<DeathLineMarker>>,
    mut commands: Commands,
    hovering_block: Res<HoveringBlock>,
    mut level: ResMut<Level>,
) {
    commands.entity(spawn_indicator.single()).despawn();
    commands.entity(death_marker.single()).despawn();
    level.insert(hovering_block.hovering, hovering_block.original_material);
    let container = SerdeMapContainer {
        map: level.material_map.clone()
    };
    let serialized = match serde_json::to_string(&container) {
        Ok(data) => data,
        Err(e) => {eprint!("{e}"); String::new()},
    };
    println!("{serialized}");
    if let Err(e) = write("assets/level.json", serialized) {
        eprintln!("Failed to save level: {}", e);
    } else {
        println!("Successfully saved!")
    }
}
