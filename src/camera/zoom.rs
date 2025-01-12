use super::CameraMarker;
use crate::utilities::easing::TimeEase;
use bevy::prelude::*;

/// The Plugin containing everything related to zooming
pub struct ZoomPlugin;

impl Plugin for ZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_zoom);
    }
}

// Systems

/// System that manages the cameras zoom
fn update_zoom(mut query: Query<(&mut OrthographicProjection, &TimeEase), With<CameraMarker>>) {
    let (mut projection, time_ease) = query.single_mut();
    projection.scale = 1. / time_ease.get_current_value();
}
