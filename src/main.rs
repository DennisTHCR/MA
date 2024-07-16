#![windows_subsystem = "windows"]
mod camera;
mod input;
mod level_management;
mod player;
mod utilities;
mod game_logic;
mod states;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use input::InputPlugin;
use level_management::LevelManagementPlugin;
use player::PlayerPlugin;
use utilities::UtilitiesPlugin;
use game_logic::GameLogicPlugin;
use states::StatePlugin;

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
            GameLogicPlugin,
            StatePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
        ))
        .run();
}
