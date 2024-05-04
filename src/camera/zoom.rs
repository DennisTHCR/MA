use super::CameraMarker;
use bevy::prelude::*;
use crate::utilities::easing::TimeEase;

pub struct ZoomPlugin;

impl Plugin for ZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_zoom);
    }
}

// Systems

/// System that changes the cameras zoom.
fn update_zoom(
    mut query: Query<(&mut OrthographicProjection, &TimeEase), With<CameraMarker>>,
) {
    let (mut projection, time_ease) = query.single_mut();
    projection.scale = 1. / time_ease.get_current_value();
}
