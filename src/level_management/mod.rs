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

// TODO: Add array/list resource containing blocks (x, y) -> Material
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
}

fn setup_level(
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    level: Res<LevelMaterials>,
) {
    for (position, material) in &level.0 {
        let position = Vec2::new(position.0 as f32 * 16. + 8., position.1 as f32 * 16. + 8.);
        if let Some(block) = material {
            commands.spawn((
                SpriteBundle {
                    texture: image_handles.0.get(&(block.clone(), Row::TOP, Column::LEFT)).unwrap().clone(),
                    transform: Transform::from_translation(position.extend(0.)),
                    ..default()
                },
                Collider::cuboid(16. / 2., 16. / 2.),
                Name::new("Textured Block"),
            ));
        }
    }
}

struct Block {
    pos: Vec2,
    size: Vec2,
    texture: Option<Handle<Image>>,
}

impl Block {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Block {
        Block {
            pos: Vec2::new(x, y),
            size: Vec2::new(width, height),
            texture: None,
        }
    }

    fn new_textured(x: f32, y: f32, width: f32, height: f32, texture: Handle<Image>) -> Block {
        Block {
            pos: Vec2::new(x, y),
            size: Vec2::new(width, height),
            texture: Some(texture),
        }
    }
}
