#![windows_subsystem = "windows"]
mod camera;
mod utilities;
mod input;
mod level_management;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use utilities::{
    UtilitiesPlugin,
    movement::follow::TargetMarker
};
use input::InputPlugin;
use level_management::LevelManagementPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraPlugin,
            UtilitiesPlugin,
            WorldInspectorPlugin::new(),
            InputPlugin,
            LevelManagementPlugin,
        ))
        .add_systems(Startup, (setup, sprite_test).chain())
        .run();
}

/// For now just themes egui to use the catppuccin theme.
fn setup(mut contexts: EguiContexts) {
    catppuccin_egui::set_theme(contexts.ctx_mut(), catppuccin_egui::MOCHA);
}

fn sprite_test(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("test.png");
    commands.spawn((
        SpriteBundle {
            texture: handle,
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        PlayerMarker,
        TargetMarker::new(0),
    ));
}

// Temporary player identifier
#[derive(Component)]
pub struct PlayerMarker;
