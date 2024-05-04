use super::CameraMarker;
use crate::utilities::easing::*;
use bevy::prelude::*;

pub struct ZoomPlugin;

impl Plugin for ZoomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Zoom::default())
            .register_type::<Zoom>()
            .add_systems(Update, (update_zoom,));
    }
}

// Systems

/// System that changes the cameras zoom.
fn update_zoom(
    mut projection: Query<&mut OrthographicProjection, With<CameraMarker>>,
    mut zoom: ResMut<Zoom>,
    time: Res<Time>,
) {
    if zoom.is_done() {
        let goal_zoom = zoom.get_end_val();
        zoom.set_previous_zoom(goal_zoom);
        return;
    }
    projection.single_mut().scale = 1. / zoom.get_current_value();
    zoom.increase_step(time.delta().as_millis() as u16);
}

// Structs

/// Resource for managing zoom level.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct Zoom(EaseStruct);

impl Default for Zoom {
    fn default() -> Self {
        Zoom(EaseStruct::new(
            1,
            1,
            1.,
            1.,
            EasingFunction::Sine,
            EasingType::InOut,
        ))
    }
}

impl Zoom {
    /// Returns whether the tweening is done or not.
    pub fn is_done(&self) -> bool {
        self.0.is_done()
    }

    /// Setter for previous zoom value.
    fn set_previous_zoom(&mut self, zoom: f32) {
        self.0.set_start_val(zoom);
    }

    /// Only way you should interact with zoom level.
    #[allow(dead_code)]
    pub fn set_zoom(
        &mut self,
        goal_zoom: f32,
        easing_function: EasingFunction,
        easing_type: EasingType,
        ms: u16,
    ) {
        *self = Zoom(EaseStruct::new(
            0,
            ms,
            self.0.get_current_value(),
            goal_zoom,
            easing_function,
            easing_type,
        ))
    }

    #[allow(dead_code)]
    pub fn force_zoom(&mut self, goal_zoom: f32) {
        self.0.force_done();
        self.0.set_end_val(goal_zoom);
    }

    pub fn get_end_val(&self) -> f32 {
        self.0.get_end_val()
    }

    pub fn get_current_value(&self) -> f32 {
        self.0.get_current_value()
    }

    pub fn increase_step(&mut self, amount: u16) {
        self.0.increase_step(amount);
    }
}
