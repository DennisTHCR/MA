use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_collider)
            .add_systems(Update, gravity);
    }
}

fn setup_collider(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(100., 5.))
        .insert(TransformBundle::from(Transform::from_xyz(0., 0., -10.)));
}

fn gravity(mut query: Query<(&Gravity, &mut Transform)>, time: Res<Time>) {

}

#[derive(Component)]
pub struct Gravity(f32);