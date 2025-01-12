use bevy::prelude::*;

pub mod assets;
pub mod easing;

pub struct UtilitiesPlugin;

/// A Plugin containing utilities
impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((easing::EasingPlugin, assets::AssetPlugin));
    }
}
