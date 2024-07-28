use std::collections::HashMap;
use std::path::Path;

use crate::utilities::assets::{init_resources, ColorResource};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::geometry::Collider;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level(Vec::new())).add_systems(
            Startup,
            (load_level, setup_level.after(init_resources)).chain(),
        );
    }
}

// TODO: Add array/list resource(/component for multiple levels?) containing positions and sizes for colliders to be added (and textures?)
// TODO: Add enum with tile names (possibly linked to texture handle immediately?)

fn load_level(mut level: ResMut<Level>, asset_server: Res<AssetServer>) {
    level.0.push(Block::new_textured(
        0.,
        0.,
        16.,
        16.,
        asset_server.load(Path::new("GRASS_GREEN/GRASS_GREEN_TOP_MIDDLE.png")),
    ));
    level.0.push(Block::new_textured(
        -16.,
        0.,
        16.,
        16.,
        asset_server.load(Path::new("GRASS_GREEN/GRASS_GREEN_TOP_LEFT.png")),
    ));
    level.0.push(Block::new_textured(
        16.,
        0.,
        16.,
        16.,
        asset_server.load(Path::new("GRASS_GREEN/GRASS_GREEN_TOP_RIGHT.png")),
    ));
    level.0.push(Block::new_textured(
        0.,
        -16.,
        16.,
        16.,
        asset_server.load(Path::new("GRASS_GREEN/GRASS_GREEN_BOTTOM_MIDDLE.png")),
    ));
    level.0.push(Block::new_textured(
        -16.,
        -16.,
        16.,
        16.,
        asset_server.load(Path::new("GRASS_GREEN/GRASS_GREEN_BOTTOM_LEFT.png")),
    ));
    level.0.push(Block::new_textured(
        16.,
        -16.,
        16.,
        16.,
        asset_server.load(Path::new("GRASS_GREEN/GRASS_GREEN_BOTTOM_RIGHT.png")),
    ));
}

fn setup_level(
    mut commands: Commands,
    materials: Res<ColorResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    level: Res<Level>,
) {
    for block in &level.0 {
        if block.texture.is_some() {
            commands.spawn((
                SpriteBundle {
                    texture: block.texture.clone().unwrap(),
                    transform: Transform::from_translation(block.pos.extend(0.)),
                    ..default()
                },
                Collider::cuboid(block.size.x / 2., block.size.y / 2.),
                Name::new("Textured Block"),
            ));
        } else {
            let mesh: Mesh2dHandle = meshes
                .add(Rectangle::new(block.size.x, block.size.y))
                .into();
            commands.spawn((
                ColorMesh2dBundle {
                    mesh: mesh.clone(),
                    material: materials.0[0].0.clone(),
                    transform: Transform::from_translation(block.pos.extend(0.)),
                    ..default()
                },
                Collider::cuboid(block.size.x / 2., block.size.y / 2.),
                Name::new("Block"),
            ));
        }
    }
}

#[derive(Resource)]
struct Level(Vec<Block>);

#[derive(Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub enum BlockMaterial {
    GRASS_GREEN,
    GRASS_ORANGE,
    GRASS_PINK,
    WOOD,
    STEEL,
    BRONZE,
    GOLD,
    BRICK,
}

#[derive(Resource)]
pub struct ImageHandles(pub HashMap<BlockMaterial, Handle<Image>>);

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
