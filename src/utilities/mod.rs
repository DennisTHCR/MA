use bevy::prelude::*;

pub mod easing;
pub mod movement;

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((easing::EasingPlugin,));
    }
}
