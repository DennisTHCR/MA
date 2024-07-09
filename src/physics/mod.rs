use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_collider)
            .add_systems(
                Update,
                (apply_gravity, update_jump_force, apply_forces, apply_offset).chain(),
            )
            .register_type::<JumpForce>()
            .register_type::<GravityForce>()
            .register_type::<Offset>()
            .register_type::<Gravity>();
    }
}

fn setup_collider(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(100., 5.))
        .insert(TransformBundle::from(Transform::from_xyz(0., 0., -10.)));
}

fn apply_offset(mut query: Query<(&Offset, &mut KinematicCharacterController, &InputOffset)>) {
    query
        .iter_mut()
        .for_each(|(offset, mut character_controller, input_offset)| {
            if character_controller.translation.is_some() {
                character_controller.translation =
                    Some(character_controller.translation.unwrap() + offset.0 + input_offset.0);
            } else {
                character_controller.translation = Some(offset.0 + input_offset.0);
            }
        });
}

fn apply_forces(mut query: Query<(&JumpForce, &GravityForce, &mut Offset)>) {
    query
        .iter_mut()
        .for_each(|(jump_force, gravity_force, mut offset)| {
            offset.0 = jump_force.0 + gravity_force.0;
        });
}

fn apply_gravity(
    mut query: Query<(
        &mut GravityForce,
        &Gravity,
        &KinematicCharacterControllerOutput,
        &KinematicCharacterController,
        &JumpForce,
    )>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(
        |(
            mut gravity_force,
            gravity,
            character_controller_output,
            character_controller,
            jump_force,
        )| {
            if !character_controller_output.grounded && jump_force.0.length() <= 0.1 {
                gravity_force.0 -= gravity.0 * character_controller.up * time.delta_seconds();
            } else {
                gravity_force.0 = Vec2::new(0., 0.);
            }
        },
    );
}

fn update_jump_force(mut query: Query<(&Gravity, &mut JumpForce)>, time: Res<Time>) {
    query.iter_mut().for_each(|(gravity, mut jump_force)| {
        let copy = jump_force.0.clone();
        jump_force.0 -= copy * gravity.0 * time.delta_seconds();
        if jump_force.0.length() <= 0.1 {
            jump_force.0 = Vec2::ZERO;
        }
    })
}

#[derive(Reflect, Component)]
pub struct Gravity(pub f32);
#[derive(Reflect, Component)]
pub struct GravityForce(pub Vec2);
#[derive(Reflect, Component)]
pub struct JumpForce(pub Vec2);
#[derive(Reflect, Component)]
pub struct Offset(pub Vec2);
#[derive(Reflect, Component)]
pub struct InputOffset(pub Vec2);
