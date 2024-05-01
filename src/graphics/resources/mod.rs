pub mod meshes;
pub mod materials;

use meshes::MeshContainerPlugin;
use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MeshContainerPlugin);
    }
}