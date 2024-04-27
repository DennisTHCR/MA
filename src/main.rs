mod graphics;
mod utilities;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use graphics::GraphicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            GraphicsPlugin,
            WorldInspectorPlugin::new(),
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
    commands.spawn(SpriteBundle {
        texture: handle,
        ..default()
    });
}
