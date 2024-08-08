use crate::utilities::assets::Material;
use crate::utilities::assets::{init, Column, ImageHandles, Row};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::geometry::Collider;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level {
            material_map: HashMap::new(),
            entity_map: HashMap::new(),
        })
        .add_systems(Startup, (load_level, setup_level).chain().after(init));
    }
}

#[derive(Resource)]
pub struct Level {
    pub material_map: HashMap<(i32, i32), Material>,
    pub entity_map: HashMap<(i32, i32), Entity>,
}

fn load_level(mut level: ResMut<Level>, mut commands: Commands, image_handles: Res<ImageHandles>) {
    level.insert((0, 0), Material::GRASS_GREEN, &mut commands, &image_handles);
    level.insert(
        (-1, 0),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert((1, 0), Material::GRASS_GREEN, &mut commands, &image_handles);
    level.insert(
        (0, -1),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert(
        (-1, -1),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert(
        (1, -1),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert(
        (0, -2),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert(
        (-1, -2),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert(
        (1, -2),
        Material::GRASS_GREEN,
        &mut commands,
        &image_handles,
    );
    level.insert((2, -2), Material::BRONZE, &mut commands, &image_handles);
    level.insert((2, -1), Material::BRONZE, &mut commands, &image_handles);
    level.insert((2, 0), Material::BRONZE, &mut commands, &image_handles);
    level.insert((3, -2), Material::BRONZE, &mut commands, &image_handles);
    level.insert((3, -1), Material::BRONZE, &mut commands, &image_handles);
    level.insert((3, 0), Material::BRONZE, &mut commands, &image_handles);
    level.insert((4, -2), Material::BRONZE, &mut commands, &image_handles);
    level.insert((4, -1), Material::BRONZE, &mut commands, &image_handles);
    level.insert((4, 0), Material::BRONZE, &mut commands, &image_handles);
}

fn setup_level(mut commands: Commands, image_handles: Res<ImageHandles>, mut level: ResMut<Level>) {
    for (position, material) in level.material_map.clone() {
        let world_position = Vec2::new(position.0 as f32 * 16. + 8., position.1 as f32 * 16. + 8.);
        let row = level.get_row(position.clone());
        let column = level.get_column(position.clone());
        let entity = commands.spawn((
            SpriteBundle {
                texture: image_handles
                    .0
                    .get(&(material.clone(), row, column))
                    .unwrap()
                    .clone(),
                transform: Transform::from_translation(world_position.extend(0.)),
                ..default()
            },
            Collider::cuboid(16. / 2., 16. / 2.),
            Name::new("Textured Block"),
        ));
        level.entity_map.insert(position.clone(), entity.id());
    }
}

impl Level {
    pub fn insert(
        &mut self,
        position: (i32, i32),
        material: Material,
        commands: &mut Commands,
        image_handles: &ImageHandles,
    ) {
        let translation = Vec3::new(
            position.0 as f32 * 16. + 8.,
            position.1 as f32 * 16. + 8.,
            10.,
        );
        self.material_map.insert(position, material.clone());
        if self.entity_map.get(&position.clone()).is_none() {
            let row = self.get_row(position.clone());
            let column = self.get_column(position.clone());
            let entity = commands.spawn((
                SpriteBundle {
                    texture: image_handles
                        .0
                        .get(&(material.clone(), row, column))
                        .expect("Couldn't find image handle for material.")
                        .clone(),
                    transform: Transform::from_translation(translation),
                    ..default()
                },
                Collider::cuboid(16. / 2., 16. / 2.),
                Name::new("Textured Block"),
            ));
            self.entity_map.insert(position.clone(), entity.id());
        }
    }

    #[allow(unused_variables)]
    pub fn remove((x, y): (i32, i32)) {}
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
