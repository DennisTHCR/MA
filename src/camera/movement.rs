use bevy::prelude::*;
use crate::utilities::easing::{TimeEase, EasingFunction, EasingType};
/*
    Rewrite:
    Camera modes: Free, Follow
    Free: fn -> Sine InOut ease delta translation
    Follow: delta player camera translation * distance
*/

/// Decides the `Camera`s movement mode.
#[derive(Component, Default)]
enum MovementMode {
    #[default]
    Follow,
    Free,
}

/// Component to mark camera as using `Follow` `MovementMode`.
#[derive(Component, Default)]
struct FollowMarker;

/// Bundle containing everything the `Free` `MovementMode` needs.
#[derive(Bundle)]
struct FreeBundle {
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