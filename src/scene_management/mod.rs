use bevy::prelude::*;
pub struct SceneManagementPlugin;

impl Plugin for SceneManagementPlugin {
    fn build(&self, app: &mut App) {
        app.world.id();
    }
}

// Bundle for a Aabb2d or BoundingCircle and asset (gizmo first) TODO
