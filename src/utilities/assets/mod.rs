use bevy::prelude::*;
use std::{collections::HashMap, path::Path};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::{player::PlayerMarker, states::AppState};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, animate_sprite.run_if(in_state(AppState::Playing)));
    }
}

#[allow(non_camel_case_types)]
#[derive(
    Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug,
)]
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

#[derive(
    Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug,
)]
pub enum Row {
    TOP,
    CENTER,
    BOTTOM,
}

#[derive(
    Component, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug,
)]
pub enum Column {
    LEFT,
    MIDDLE,
    RIGHT,
}

pub fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
                    let handle: Handle<Image> =
                        asset_server.load(Path::new(material.as_ref()).join(format!(
                            "{}_{}_{}.png",
                            material.as_ref(),
                            row.as_ref(),
                            column.as_ref()
                        )));
                    image_handles.0.insert((material, row, column), handle);
                }
            })
        })
    });
    commands.insert_resource(image_handles);
    let mut player_animations: PlayerAnimationMap = PlayerAnimationMap(HashMap::new());
    PlayerAnimationState::iter().for_each(|animation| {
        let columns = match animation {
            PlayerAnimationState::FALL => 1,
            PlayerAnimationState::IDLE => 11,
            PlayerAnimationState::JUMP => 1,
            PlayerAnimationState::RUN => 12,
        };
        let texture: Handle<Image> = asset_server.load(
            Path::new("CHARACTER")
                .join(animation.as_ref())
                .with_extension("png"),
        );
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), columns, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let anim = PlayerAnimation {
            texture,
            texture_atlas_layout,
        };
        player_animations.0.insert(animation, anim);
    });
    commands.insert_resource(player_animations);
    commands.insert_resource(PlayerAnimationState::IDLE);
    commands.insert_resource(PlayerAnimationIndices { first: 0, last: 10 });
    commands.insert_resource(PlayerAnimationTimer(Timer::from_seconds(
        0.05,
        TimerMode::Repeating,
    )));
}

#[derive(Resource)]
pub struct ColorResource(pub [(Handle<ColorMaterial>, String, Color); 2]);

#[derive(Resource)]
pub struct PlayerAnimationMap(pub HashMap<PlayerAnimationState, PlayerAnimation>);

pub struct PlayerAnimation {
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(
    Resource, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, EnumIter, AsRefStr, Debug,
)]
pub enum PlayerAnimationState {
    FALL,
    IDLE,
    JUMP,
    RUN,
}

#[derive(Resource)]
pub struct PlayerAnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerAnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut timer: ResMut<PlayerAnimationTimer>,
    indices: Res<PlayerAnimationIndices>,
    mut query: Query<&mut TextureAtlas, With<PlayerMarker>>,
) {
    let mut atlas = query.single_mut();
    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = if atlas.index >= indices.last {
            indices.first
        } else {
            atlas.index + 1
        };
    }
}
