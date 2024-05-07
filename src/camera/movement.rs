use crate::utilities::easing::{EasingFunction, EasingType, TimeEase};
use bevy::prelude::*;
use std::collections::HashMap;
/*
    Rewrite:
    Camera modes: Free, Follow
    Free: fn -> Sine InOut ease delta translation
    Follow: delta player camera translation * distance
*/

/// Plugin to manage camera movement.
pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_time_ease,
                (following_movement_system, free_movement_system),
            )
                .chain(),
        );
    }
}

/// Enum to decide the `Camera`s movement mode.
#[derive(Component, Default)]
pub enum MovementMode {
    #[default]
    Follow,
    Free,
}

// Follow Mode Section

/// Component to mark camera as using `Follow` `MovementMode`. i32 is used as ID to link to target.
#[derive(Component)]
pub struct FollowMarker(i32);

impl FollowMarker {
    pub fn new(id: i32) -> Self {
        FollowMarker(id)
    }
}

/// Component to mark the entity to follow. i32 is used as ID to link to following camera.
#[derive(Component)]
pub struct TargetMarker(i32);

impl TargetMarker {
    pub fn new(id: i32) -> Self {
        TargetMarker(id)
    }
}

/// System that pulls the following entity towards its target. Speed depends on delta transform.
fn following_movement_system(
    mut follower: Query<(&FollowMarker, &mut Transform), Without<TargetMarker>>,
    target: Query<(&TargetMarker, &Transform), Without<FollowMarker>>,
    time: Res<Time>,
) {
    let mut map = HashMap::new();
    target.iter().for_each(|(marker, transform)| {
        map.insert(marker.0, transform);
    });
    follower.iter_mut().for_each(|(marker, mut transform)| {
        let target_transform = **map.get(&marker.0).unwrap();
        let delta = target_transform.translation - transform.translation;
        transform.translation += delta / 3. * time.delta_seconds();
    })
}

// Free Mode Section

/// Component to mark camera as using `Free` `MovementMode`
#[derive(Component)]
struct FreeMarker;

#[derive(Component)]
struct StartGoalTransform {
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

fn free_movement_system(mut query: Query<(&TimeEase, &StartGoalTransform, &mut Transform)>) {
    query
        .iter_mut()
        .for_each(|(time_ease, start_goal_transform, mut transform)| {
            let progress = time_ease.get_current_value();
            let delta = start_goal_transform.get_delta();
            transform.translation = start_goal_transform.get_start() + delta * progress;
        })
}

fn update_time_ease(
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
