use crate::utilities::assets::{init, ImageHandles, Row, Column};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::geometry::Collider;
use crate::utilities::assets::Material;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelMaterials(HashMap::new()))
            .add_systems(Startup, (load_level, setup_level.after(init)).chain());
    }
}

#[derive(Resource)]
pub struct LevelMaterials(pub HashMap<(i32, i32), Option<Material>>);
#[derive(Resource)]
pub struct LevelEntities(pub HashMap<(i32, i32), Entity>);

fn load_level(mut level: ResMut<LevelMaterials>) {
    level.0.insert((0,0), Some(Material::GRASS_GREEN));
    level.0.insert((-1,0), Some(Material::GRASS_GREEN));
    level.0.insert((1,0), Some(Material::GRASS_GREEN));
    level.0.insert((0,-1), Some(Material::GRASS_GREEN));
    level.0.insert((-1, -1), Some(Material::GRASS_GREEN));
    level.0.insert((1,-1), Some(Material::GRASS_GREEN));
    level.0.insert((0,-2), Some(Material::GRASS_GREEN));
    level.0.insert((-1, -2), Some(Material::GRASS_GREEN));
    level.0.insert((1,-2), Some(Material::GRASS_GREEN));
    level.0.insert((2, -2), Some(Material::BRONZE));
    level.0.insert((2, -1), Some(Material::BRONZE));
    level.0.insert((2, 0), Some(Material::BRONZE));
    level.0.insert((3, -2), Some(Material::BRONZE));
    level.0.insert((3, -1), Some(Material::BRONZE));
    level.0.insert((3, 0), Some(Material::BRONZE));
    level.0.insert((4, -2), Some(Material::BRONZE));
    level.0.insert((4, -1), Some(Material::BRONZE));
    level.0.insert((4, 0), Some(Material::BRONZE));
}

fn setup_level(
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    level: ResMut<LevelMaterials>,
) {
    for (position, material) in &level.0 {
        let world_position = Vec2::new(position.0 as f32 * 16. + 8., position.1 as f32 * 16. + 8.);
        if let Some(block) = material {
            let row = level.get_row(position.clone(), block);
            let column = level.get_column(position.clone(), block);
            commands.spawn((
                SpriteBundle {
                    texture: image_handles.0.get(&(block.clone(), row, column)).unwrap().clone(),
                    transform: Transform::from_translation(world_position.extend(0.)),
                    ..default()
                },
                Collider::cuboid(16. / 2., 16. / 2.),
                Name::new("Textured Block"),
            ));
        }
    }
}

impl LevelMaterials {
    fn get_row(&self, (x,y): (i32, i32), material: &Material) -> Row {
        let mut top = self.0.get(&(x, y + 1)).is_some();
        if top {
            top = self.0.get(&(x, y + 1)).unwrap().is_some();
        }
        if top {
            let top_material = self.0.get(&(x, y + 1)).unwrap().unwrap();
            top = &top_material == material;
        }
        let mut bottom = self.0.get(&(x, y - 1)).is_some();
        if bottom {
            bottom = self.0.get(&(x, y - 1)).unwrap().is_some();
        }
        if bottom {
            let bottom_material = self.0.get(&(x, y - 1)).unwrap().unwrap();
            bottom = &bottom_material == material;
        }
        let row = match (top,bottom) {
            (false, false) => Row::CENTER,
            (true, false) => Row::BOTTOM,
            (false, true) => Row::TOP,
            (true, true) => Row::CENTER,
        };
        if !material.is_small() || row == Row::TOP {
            return row;
        }
        return match self.get_row((x, y + 1), &self.0.get(&(x, y + 1)).unwrap().unwrap()) {
            Row::TOP => Row::BOTTOM,
            Row::BOTTOM => Row::TOP,
            Row::CENTER => Row::BOTTOM
        }
    }

    fn get_column(&self, (x,y): (i32, i32), material: &Material) -> Column {
        let mut left = self.0.get(&(x - 1, y)).is_some();
        if left {
            left = self.0.get(&(x - 1, y)).unwrap().is_some();
        }
        if left {
            let left_material = self.0.get(&(x - 1, y)).unwrap().unwrap();
            left = &left_material == material;
        }
        let mut right = self.0.get(&(x + 1, y)).is_some();
        if right {
            right = self.0.get(&(x + 1, y)).unwrap().is_some();
        }
        if right {
            let right_material = self.0.get(&(x + 1, y)).unwrap().unwrap();
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
        return match self.get_column((x - 1, y), &self.0.get(&(x - 1, y)).unwrap().unwrap()) {
            Column::LEFT => Column::RIGHT,
            Column::RIGHT => Column::LEFT,
            Column::MIDDLE => Column::RIGHT,
        }
    }
}