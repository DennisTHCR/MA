use crate::utilities::easing::{EasingFunction, EasingType, TimeEase};
use bevy::prelude::*;

/// Component to mark entity as using `Free` `MovementMode`
#[derive(Component)]
struct FreeMarker;

#[derive(Component)]
pub struct StartGoalTransform {
    start_transform: Transform,
    goal_transform: Transform,
}

impl StartGoalTransform {
    pub fn get_delta(&self) -> Vec3 {
        self.goal_transform.translation - self.start_transform.translation
    }

    pub fn get_start(&self) -> Vec3 {
        self.start_transform.translation
    }

    pub fn set_start(&mut self, start_transform: Transform) {
        self.start_transform = start_transform;
    }
}

/// Bundle containing everything the `Free` `MovementMode` needs.
#[derive(Bundle)]
struct FreeBundle {
    free_marker: FreeMarker,
    time_ease: TimeEase,
    start_goal_transform: StartGoalTransform,
}

impl Default for FreeBundle {
    fn default() -> Self {
        FreeBundle {
            time_ease: TimeEase::new(0, 1000, 0., 1., EasingFunction::Sine, EasingType::InOut),
            ..default()
        }
    }
}

/// System that moves entities along their path.
pub fn free_movement_system(mut query: Query<(&TimeEase, &StartGoalTransform, &mut Transform)>) {
    query
        .iter_mut()
        .for_each(|(time_ease, start_goal_transform, mut transform)| {
            let progress = time_ease.get_current_value();
            let delta = start_goal_transform.get_delta();
            transform.translation = start_goal_transform.get_start() + delta * progress;
        })
}

/// System that handles changes made to the StartGoalTransform value
pub fn update_time_ease(
    mut query: Query<
        (&mut TimeEase, &mut StartGoalTransform, &Transform),
        Changed<StartGoalTransform>,
    >,
) {
    query
        .iter_mut()
        .for_each(|(mut time_ease, mut start_goal_transform, transform)| {
            time_ease.set_step(0);
            start_goal_transform.set_start(*transform);
        })
}
