use bevy::prelude::*;
use crate::utilities::easing::{TimeEase, EasingFunction, EasingType};
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
        app
            .add_systems(Update, following_movement_system);
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
fn following_movement_system(mut follower: Query<(&FollowMarker, &mut Transform), Without<TargetMarker>>, target: Query<(&TargetMarker, &Transform), Without<FollowMarker>>, time: Res<Time>) {
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

/// Bundle containing everything the `Free` `MovementMode` needs.
#[derive(Bundle)]
struct FreeBundle {
    free_marker: FreeMarker,
    time_ease: TimeEase,
    start_transform: Transform,
    goal_transform: Transform,
}

impl Default for FreeBundle {
    fn default() -> Self {
        FreeBundle {
            time_ease: TimeEase::new(0, 1000, 0., 1., EasingFunction::Sine, EasingType::InOut),
            ..default()
        }
    }
}