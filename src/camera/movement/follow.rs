use bevy::{prelude::*, window::PrimaryWindow};
use std::collections::HashMap;

#[derive(Component)]
pub struct FollowMarker(Target);

impl FollowMarker {
    pub fn new(target: Target) -> Self {
        FollowMarker(target)
    }
}

/// Component to mark the entity to follow. i32 is used as ID to link for following entities.
#[derive(Component)]
pub struct TargetMarker(Target);

impl TargetMarker {
    pub fn new(target: Target) -> Self {
        TargetMarker(target)
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Target {
    Player
}

/// System that pulls the following entity towards its target. Speed depends on delta transform.
pub fn following_movement_system(
    mut follower: Query<(&FollowMarker, &mut Transform), Without<TargetMarker>>,
    target: Query<(&TargetMarker, &Transform), Without<FollowMarker>>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let width = window.single().width();
    let height = window.single().height();
    let mut map = HashMap::new();
    target.iter().for_each(|(marker, transform)| {
        map.insert(marker.0, transform);
    });
    follower.iter_mut().for_each(|(marker, mut transform)| {
        let target_transform = **map.get(&marker.0).unwrap();
        let delta = target_transform.translation - transform.translation;
        transform.translation += delta * time.delta_seconds();
        if delta.x.abs() > 0.5 * width {
            transform.translation.x += (delta.x.abs() - 0.5 * width) * delta.x.abs() / delta.x;
        }
        if delta.y.abs() > 0.5 * height {
            transform.translation.y += (delta.y.abs() - 0.5 * height) * delta.y.abs() / delta.y;
        }
        if transform.translation.x == f32::INFINITY || transform.translation.y == f32::INFINITY {
            transform.translation = target_transform.translation;
        }
    });
}
