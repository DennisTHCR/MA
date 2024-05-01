use crate::graphics::resources::{materials, meshes};
use materials::MaterialContainer;
use meshes::MeshContainer;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle,};
pub struct SceneManagementPlugin;

impl Plugin for SceneManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, meshes: ResMut<MeshContainer>, materials: ResMut<MaterialContainer>) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.square.clone(),
        material: materials.white.clone(),
        ..default()
    });
}

// Bundle for a Aabb2d or BoundingCircle and asset (gizmo first) TODO
