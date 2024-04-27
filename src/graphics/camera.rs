use crate::utilities::easing::*;
use bevy::prelude::*;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (Camera2dBundle::default(), CameraMarker);

        app.world.spawn(bundle);

        app.insert_resource(GoalZoom::default())
            .register_type::<GoalZoom>()
            .insert_resource(Zoom::default())
            .register_type::<Zoom>()
            .insert_resource(ZoomEase::default())
            .add_systems(Update, (update_zoom_resource, update_zoom).chain());
    }
}

// Systems

/// System that changes Zoom level.
fn update_zoom(
    mut projection: Query<&mut OrthographicProjection, With<CameraMarker>>,
    zoom: Res<Zoom>,
) {
    projection.single_mut().scale = zoom.0;
}

/// System that updates the Zoom resource to zoom smoothly. TODO: Add smoothing function (for now just updates the zoom immediately)
fn update_zoom_resource(mut zoom: ResMut<Zoom>, goal_zoom: Res<GoalZoom>, mut zoom_ease: ResMut<ZoomEase>, time: Res<Time>) {
    zoom.0 = 1. / (goal_zoom.0 * zoom_ease.0.progress_eased());
    zoom_ease.0.increase(time.delta().as_millis() as u16);
}

// Components and Resources

/// Marks the main camera.
#[derive(Component)]
pub struct CameraMarker;

/// Sets zoom level immediately. Should not be accesssed directly.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct Zoom(f32);

impl Default for Zoom {
    fn default() -> Self {
        Zoom(1.)
    }
}

/// Changes Zoom level with easing function.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct GoalZoom(f32);

impl Default for GoalZoom {
    fn default() -> Self {
        GoalZoom(Zoom::default().0)
    }
}

#[derive(Resource)]
struct ZoomEase(EaseStruct);

impl Default for ZoomEase {
    fn default() -> Self {
        ZoomEase(
            EaseStruct {
                current_step: 0,
                total_steps: 500,
                easing_function: EasingFunction::Back,
                easing_type: EasingType::Out,
            }
        )
    }
}
