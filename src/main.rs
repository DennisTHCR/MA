mod camera;
mod config;
mod game_logic;
mod input;
mod level_management;
mod player;
mod states;
mod utilities;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use config::ConfigPlugin;
use game_logic::GameLogicPlugin;
use input::InputPlugin;
use level_management::LevelManagementPlugin;
use player::PlayerPlugin;
use states::StatePlugin;
use utilities::UtilitiesPlugin;

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
            ConfigPlugin,
        ))
        .run();
}
