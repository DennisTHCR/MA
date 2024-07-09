use bevy::{prelude::*, window::PrimaryWindow};
use std::collections::HashMap;

#[derive(Component)]
pub struct FollowMarker(i32);

impl FollowMarker {
    pub fn new(id: i32) -> Self {
        FollowMarker(id)
    }
}

/// Component to mark the entity to follow. i32 is used as ID to link for following entities.
#[derive(Component)]
pub struct TargetMarker(i32);

impl TargetMarker {
    pub fn new(id: i32) -> Self {
        TargetMarker(id)
    }
}

/// System that pulls the following entity towards its target. Speed depends on delta transform.
pub fn following_movement_system(
    mut follower: Query<(&FollowMarker, &mut Transform), Without<TargetMarker>>,
    target: Query<(&TargetMarker, &Transform), Without<FollowMarker>>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let width = window.single().width();
    let mut map = HashMap::new();
    target.iter().for_each(|(marker, transform)| {
        map.insert(marker.0, transform);
    });
    follower.iter_mut().for_each(|(marker, mut transform)| {
        let target_transform = **map.get(&marker.0).unwrap();
        let delta = target_transform.translation - transform.translation;
        transform.translation += delta * time.delta_seconds();
        if delta.xy().length() >= 0.5 * width {
            transform.translation += delta.xy().length() - 0.5 * width;
        }
    });
}
