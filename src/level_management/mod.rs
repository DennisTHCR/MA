use crate::states::editing::{HoveringBlock, SerdeMapContainer};
use crate::utilities::assets::Material;
use crate::utilities::assets::{init, Column, ImageHandles, Row};
use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

/// The Plugin containing everything related to the [Level]
pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level {
            material_map: HashMap::new(),
            entity_map: HashMap::new(),
            level_spawn_queue: HashSet::new(),
            level_despawn_queue: HashSet::new(),
            texture_update_queue: HashSet::new(),
        })
        .add_systems(Startup, load_level.after(init))
        .add_systems(Update, execute_level_queues);
    }
}

/// A Resource containing all runtime data for the Level
#[derive(Resource, Debug)]
pub struct Level {
    pub material_map: HashMap<(i32, i32), Material>,
    pub entity_map: HashMap<(i32, i32), Entity>,
    pub level_spawn_queue: HashSet<(i32, i32)>,
    pub level_despawn_queue: HashSet<(i32, i32)>,
    pub texture_update_queue: HashSet<(i32, i32)>,
}

/// Helper function to load the level from its file
fn load_level(mut level: ResMut<Level>, mut hovering_block: ResMut<HoveringBlock>) {
    let json = read_to_string("assets/level.json");
    if !json.is_err() {
        let deserialized_option = serde_json::from_str(&json.unwrap());
        if !deserialized_option.is_err() {
            let container: SerdeMapContainer = deserialized_option.unwrap();
            level.material_map = container.map;
        }
    }
    hovering_block.original_material = match level.material_map.get(&hovering_block.last_hovered) {
        Some(material) => Some(*material),
        None => None,
    };
    for (pos, material) in level.material_map.clone() {
        level.insert(pos, Some(material));
    }
}

/// System that executes all deferred actions related to the [Level]
pub fn execute_level_queues(
    mut commands: Commands,
    mut level: ResMut<Level>,
    image_handles: Res<ImageHandles>,
) {
    level
        .level_despawn_queue
        .clone()
        .iter()
        .for_each(|&position| {
            let id = level.entity_map.get(&position);
            if id.is_some() {
                commands.entity(id.unwrap().clone()).despawn();
                level.level_despawn_queue.remove(&position);
                level.entity_map.remove(&position);
            }
        });

    level
        .level_spawn_queue
        .clone()
        .iter()
        .for_each(|&position| {
            let material = level.material_map.get(&position);
            let row = level.get_row(position);
            let column = level.get_column(position);
            let world_position =
                Vec2::new(position.0 as f32 * 16. + 8., position.1 as f32 * 16. + 8.);
            if material.is_some() {
                let texture = image_handles
                    .0
                    .get(&(material.unwrap().clone(), row, column))
                    .unwrap()
                    .clone();
                let entity = commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_translation(world_position.extend(0.)),
                        ..default()
                    },
                    Collider::cuboid(16. / 2., 16. / 2.),
                    Name::new("Textured Block"),
                ));
                level.entity_map.insert(position, entity.id());
            }
            level.level_spawn_queue.remove(&position);
        });
    level
        .texture_update_queue
        .clone()
        .iter()
        .for_each(|&position| {
            let material = level.material_map.get(&position);
            let row = level.get_row(position);
            let column = level.get_column(position);
            let id = level.entity_map.get(&position);
            if material.is_some() {
                let texture = image_handles
                    .0
                    .get(&(material.unwrap().clone(), row, column))
                    .unwrap()
                    .clone();
                if id.is_some() {
                    commands.entity(id.unwrap().clone()).insert(texture);
                }
            }
            level.texture_update_queue.remove(&position);
        });
}

impl Level {
    /// Adds or changes the Material at the given location
    pub fn insert(&mut self, position: (i32, i32), material: Option<Material>) {
        if material.is_none() {
            self.remove(position);
            return;
        }
        self.material_map
            .insert(position, material.unwrap().clone());
        self.texture_update_queue.insert(position);
        if self.entity_map.get(&position.clone()).is_none() {
            self.level_spawn_queue.insert(position);
        }
    }

    /// Removes the Material at the given location
    pub fn remove(&mut self, (x, y): (i32, i32)) {
        self.material_map.remove(&(x, y));
        self.level_despawn_queue.insert((x, y));
    }

    /// Helper function for grid
    pub fn get_row(&self, (x, y): (i32, i32)) -> Row {
        if self.material_map.get(&(x, y)).is_none() {
            return Row::TOP;
        }
        let material = self.material_map.get(&(x, y)).unwrap();
        let mut top = self.material_map.get(&(x, y + 1)).is_some();
        if top {
            let top_material = *self.material_map.get(&(x, y + 1)).unwrap();
            top = &top_material == material;
        }
        let mut bottom = self.material_map.get(&(x, y - 1)).is_some();
        if bottom {
            let bottom_material = *self.material_map.get(&(x, y - 1)).unwrap();
            bottom = &bottom_material == material;
        }
        let row = match (top, bottom) {
            (false, false) => Row::CENTER,
            (true, false) => Row::BOTTOM,
            (false, true) => Row::TOP,
            (true, true) => Row::CENTER,
        };
        if !material.is_small() || row == Row::TOP {
            return row;
        }
        return match self.get_row((x, y + 1)) {
            Row::TOP => Row::BOTTOM,
            Row::BOTTOM => Row::TOP,
            Row::CENTER => Row::BOTTOM,
        };
    }

    /// Helper function for grid
    pub fn get_column(&self, (x, y): (i32, i32)) -> Column {
        if self.material_map.get(&(x, y)).is_none() {
            return Column::LEFT;
        }
        let material = self.material_map.get(&(x, y)).unwrap();
        let mut left = self.material_map.get(&(x - 1, y)).is_some();
        if left {
            let left_material = *self.material_map.get(&(x - 1, y)).unwrap();
            left = &left_material == material;
        }
        let mut right = self.material_map.get(&(x + 1, y)).is_some();
        if right {
            let right_material = *self.material_map.get(&(x + 1, y)).unwrap();
            right = &right_material == material;
        }
        let column = match (left, right) {
            (false, false) => Column::MIDDLE,
            (true, false) => Column::RIGHT,
            (false, true) => Column::LEFT,
            (true, true) => Column::MIDDLE,
        };
        if !material.is_small() || column == Column::LEFT {
            return column;
        }
        return match self.get_column((x - 1, y)) {
            Column::LEFT => Column::RIGHT,
            Column::RIGHT => Column::LEFT,
            Column::MIDDLE => Column::RIGHT,
        };
    }
}
