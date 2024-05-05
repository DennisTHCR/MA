use super::easing::*;
use bevy::prelude::*;

// Components defining movement characteristics

#[derive(Component, Reflect)]
pub struct MaxAcceleration(f32);

#[derive(Component, Default, Reflect)]
pub struct CurrentAcceleration(pub Vec2);

#[derive(Component, Reflect)]
pub struct MaxDeceleration(f32);

#[derive(Component, Default, Reflect)]
pub struct CurrentDeceleration(Vec2);

#[derive(Component, Reflect)]
pub struct MaxVelocity(f32);

#[derive(Component, Default, Reflect)]
pub struct CurrentVelocity(Vec2);

#[derive(Component, Reflect)]
pub struct DecelerationEase(EaseStruct);

// bundle + constructor
#[derive(Bundle)]
pub struct MovementBundle {
    current_acceleration: CurrentAcceleration,
    max_acceleration: MaxAcceleration,
    current_deceleration: CurrentDeceleration,
    max_deceleration: MaxDeceleration,
    max_velocity: MaxVelocity,
    current_velocity: CurrentVelocity,
    deceleration_ease: DecelerationEase,
}

impl Default for MovementBundle {
    fn default() -> Self {
        MovementBundle {
            max_acceleration: MaxAcceleration(1.),
            max_deceleration: MaxDeceleration(1.),
            max_velocity: MaxVelocity(1.),
            deceleration_ease: DecelerationEase(EaseStruct::new(
                0,
                1,
                0.,
                1.,
                EasingFunction::default(),
                EasingType::default(),
            )),
            current_acceleration: CurrentAcceleration::default(),
            current_deceleration: CurrentDeceleration::default(),
            current_velocity: CurrentVelocity::default()
        }
    }
}

impl MovementBundle {
    pub fn new(max_acceleration: f32, max_deceleration: f32, max_velocity: f32) -> Self {
        MovementBundle {
            max_acceleration: MaxAcceleration(max_acceleration),
            max_deceleration: MaxDeceleration(max_deceleration),
            max_velocity: MaxVelocity(max_velocity),
            ..default()
        }
    }
}

// plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_accel_decel, update_deceleration, clamp_values, apply_velocity).chain(),
        )
        .register_type::<MaxAcceleration>()
        .register_type::<CurrentAcceleration>()
        .register_type::<MaxDeceleration>()
        .register_type::<CurrentDeceleration>()
        .register_type::<MaxVelocity>()
        .register_type::<CurrentVelocity>()
        .register_type::<DecelerationEase>();
    }
}

// systems
fn apply_accel_decel(
    mut query: Query<(
        &mut CurrentVelocity,
        &CurrentAcceleration,
        &CurrentDeceleration,
    )>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(mut current_velocity, current_acceleration, current_deceleration)| {
            current_velocity.0 +=
                time.delta_seconds() * (current_acceleration.0 - current_deceleration.0);
        },
    )
}

fn update_deceleration(
    mut query: Query<(
        &mut CurrentDeceleration,
        &MaxDeceleration,
        &CurrentVelocity,
        &MaxVelocity,
        &mut DecelerationEase,
    )>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(
            mut current_deceleration,
            max_deceleration,
            current_velocity,
            max_velocity,
            mut deceleration_ease,
        )| {
            let ease_struct = &mut deceleration_ease.0;
            ease_struct.set_end_val(max_velocity.0);
            ease_struct.set_step(current_velocity.0.length() as u16);
            current_deceleration.0 = current_velocity.0
                * ease_struct.get_current_value()
                * max_deceleration.0
                * time.delta_seconds();
        },
    );
}

fn clamp_values(
    mut query: Query<(
        &mut CurrentAcceleration,
        &mut CurrentDeceleration,
        &mut CurrentVelocity,
        &MaxAcceleration,
        &MaxDeceleration,
        &MaxVelocity,
    )>,
) {
    query.iter_mut().for_each(
        |(
            mut current_acceleration,
            mut current_deceleration,
            mut current_velocity,
            max_acceleration,
            max_deceleration,
            max_velocity,
        )| {
            if current_acceleration.0.length() > max_acceleration.0 {
                current_acceleration.0 = current_acceleration.0.normalize() * max_acceleration.0
            };
            if current_deceleration.0.length() > max_deceleration.0 {
                current_deceleration.0 = current_deceleration.0.normalize() * max_deceleration.0
            };
            if current_velocity.0.length() > max_velocity.0 {
                current_velocity.0 = current_velocity.0.normalize() * max_velocity.0
            };
        },
    );
}

fn apply_velocity(mut query: Query<(&CurrentVelocity, &mut Transform)>) {
    query.iter_mut().for_each(|(current_velocity, mut transform)| {
        transform.translation += Vec3::new(current_velocity.0.x, current_velocity.0.y, 0.);
    })
}
