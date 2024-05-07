mod camera;
mod utilities;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::{movement::TargetMarker, CameraMarker, CameraPlugin};
use utilities::{
    easing::{EasingFunction, EasingType, TimeEase},
    UtilitiesPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraPlugin,
            UtilitiesPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup, sprite_test).chain())
        .add_systems(Update, zoom_test)
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
            ..default()
        },
        PlayerMarker,
        TargetMarker::new(0),
    ));
}

fn zoom_test(mut query: Query<&mut TimeEase, With<CameraMarker>>) {
    let time_ease = &mut query.single_mut();
    if time_ease.is_done() {
        if time_ease.get_end_val() == 5. {
            time_ease.set_ease(10., EasingFunction::Sine, EasingType::InOut, 1000);
        } else {
            time_ease.set_ease(5., EasingFunction::Sine, EasingType::InOut, 1000);
        }
    }
}

// Temporary player identifier
#[derive(Component)]
pub struct PlayerMarker;
