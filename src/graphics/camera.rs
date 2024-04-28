use crate::utilities::easing::*;
use bevy::prelude::*;

/// Plugin that handles everything related to managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let bundle = (Camera2dBundle::default(), CameraMarker);

        app.world.spawn(bundle);

        app.insert_resource(ZoomEase::default())
            .register_type::<ZoomEase>()
            .insert_resource(Zoom::default())
            .register_type::<Zoom>()
            .insert_resource(ZoomEase::default())
            .add_systems(Update, (update_zoom_resource, update_zoom).chain());
    }
}

// Systems

/// System that changes the cameras zoom.
fn update_zoom(
    mut projection: Query<&mut OrthographicProjection, With<CameraMarker>>,
    zoom: Res<Zoom>,
) {
    projection.single_mut().scale = zoom.0;
}

/// System that updates the Zoom resource to zoom smoothly.
fn update_zoom_resource(mut zoom: ResMut<Zoom>, mut zoom_ease: ResMut<ZoomEase>, time: Res<Time>) {
    if zoom_ease.is_done() {
        let goal_zoom = zoom_ease.goal_zoom;
        zoom_ease.set_previous_zoom(goal_zoom);
        return;
    }
    zoom.0 = 1.
        / (zoom_ease.previous_zoom
            + (zoom_ease.goal_zoom - zoom_ease.previous_zoom)
                * zoom_ease.ease_struct.progress_eased());
    zoom_ease
        .ease_struct
        .increase(time.delta().as_millis() as u16);
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

/// Struct to handle zooming. Should only be accessed by implemented functions.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct ZoomEase {
    ease_struct: EaseStruct,
    goal_zoom: f32,
    previous_zoom: f32,
}

impl Default for ZoomEase {
    fn default() -> Self {
        ZoomEase {
            ease_struct: EaseStruct {
                current_step: 0,
                total_steps: 500,
                easing_function: EasingFunction::Sine,
                easing_type: EasingType::InOut,
            },
            goal_zoom: Zoom::default().0,
            previous_zoom: Zoom::default().0,
        }
    }
}

impl ZoomEase {
    /// Returns whether the tweening is done or not.
    pub fn is_done(&self) -> bool {
        self.ease_struct.current_step == self.ease_struct.total_steps
    }

    /// Setter for previous zoom value.
    fn set_previous_zoom(&mut self, zoom: f32) {
        self.previous_zoom = zoom;
    }

    /// Only way you should interact with zoom level.
    pub fn set_zoom(&mut self, goal_zoom: f32, easing_function: EasingFunction, easing_type: EasingType, time: u16) {
        *self = ZoomEase {
            ease_struct: EaseStruct {
                current_step: 0,
                total_steps: time,
                easing_function: easing_function,
                easing_type: easing_type,
            },
            goal_zoom,
            previous_zoom: self.goal_zoom,
        }
    }
}
