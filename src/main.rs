#![windows_subsystem = "windows"]
mod camera;
mod input;
mod level_management;
mod player;
mod utilities;
mod physics;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use input::InputPlugin;
use level_management::LevelManagementPlugin;
use utilities::UtilitiesPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraPlugin,
            UtilitiesPlugin,
            WorldInspectorPlugin::new(),
            InputPlugin,
            LevelManagementPlugin,
            PlayerPlugin,
            PhysicsPlugin,
        ))
        .add_systems(Startup, (setup).chain())
        .run();
}

/// For now just themes egui to use the catppuccin theme.
fn setup(mut contexts: EguiContexts) {
    catppuccin_egui::set_theme(contexts.ctx_mut(), catppuccin_egui::MOCHA);
}
