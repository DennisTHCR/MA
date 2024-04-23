use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (Camera2dBundle::default(), CameraMarker);
        app.world.spawn(bundle);
    }
}

#[derive(Component)]
pub struct CameraMarker;