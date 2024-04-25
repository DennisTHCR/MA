mod graphics;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use graphics::GraphicsPlugin;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        GraphicsPlugin,
        WorldInspectorPlugin::new(),
    ))
    .add_systems(Startup, sprite_test)
    .run();
}

fn sprite_test(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("test.png");
    commands.spawn(SpriteBundle {
        texture: handle,
        ..default()
    });
}