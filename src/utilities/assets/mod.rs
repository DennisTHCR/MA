use bevy::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

#[allow(non_camel_case_types)]
#[derive(Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug)]
pub enum Material {
    GRASS_GREEN,
    GRASS_ORANGE,
    GRASS_PINK,
    WOOD,
    STEEL,
    BRONZE,
    GOLD,
    BRICK,
}

impl Material {
    pub fn is_small(&self) -> bool {
        SMALL_MATERIALS.contains(self)
    }
}

pub const SMALL_MATERIALS: [Material; 4] = [
    Material::WOOD,
    Material::STEEL,
    Material::BRONZE,
    Material::GOLD,
];

#[derive(Resource)]
pub struct ImageHandles(pub HashMap<(Material, Row, Column), Handle<Image>>);

impl Default for ImageHandles {
    fn default() -> Self {
        ImageHandles(HashMap::new())
    }
}

#[derive(Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug)]
pub enum Row {
    TOP,
    CENTER,
    BOTTOM,
}

#[derive(Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug)]
pub enum Column {
    LEFT,
    MIDDLE,
    RIGHT,
}

pub fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let material_white = materials.add(ColorMaterial::from(Color::WHITE));
    let material_red = materials.add(ColorMaterial::from(Color::linear_rgb(150., 0., 0.)));
    commands.insert_resource(ColorResource([
        (material_white, "WHITE".to_string(), Color::WHITE),
        (
            material_red,
            "RED".to_string(),
            Color::linear_rgb(150., 0., 0.),
        ),
    ]));
    let mut image_handles: ImageHandles = ImageHandles::default();
    Material::iter().for_each(|material| {
        Column::iter().for_each(|column| {
            Row::iter().for_each(|row| {
                if (row != Row::CENTER && column != Column::MIDDLE)
                    || !SMALL_MATERIALS.contains(&material)
                {
                    let handle: Handle<Image> = asset_server.load(
                        String::from(material.as_ref())
                            + "/"
                            + material.as_ref()
                            + "_"
                            + row.as_ref()
                            + "_"
                            + column.as_ref()
                            + ".png",
                    );
                    image_handles.0.insert((material, row, column), handle);
                }
            })
        })
    });
    commands.insert_resource(image_handles);
}

#[derive(Resource)]
pub struct ColorResource(pub [(Handle<ColorMaterial>, String, Color); 2]);
