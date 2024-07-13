use crate::utilities::assets::{init_resources, ColorResource};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::geometry::Collider;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level.after(init_resources));
    }
}

// TODO: Add array/list resource(/component for multiple levels?) containing positions and sizes for colliders to be added (and textures?)

fn setup_level(
    mut commands: Commands,
    materials: Res<ColorResource>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh: Mesh2dHandle = meshes.add(Rectangle::new(2000., 100.)).into();
    commands.spawn((
        ColorMesh2dBundle {
            mesh: mesh.clone(),
            material: materials.0[0].0.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Collider::cuboid(1000., 50.),
        Name::new("Floor 1")
    ));
    commands.spawn((
        ColorMesh2dBundle {
            mesh: mesh.clone(),
            material: materials.0[0].0.clone(),
            transform: Transform::from_xyz(2200., 10., 0.),
            ..default()
        },
        Collider::cuboid(1000., 50.),
        Name::new("Floor 2")
    ));
}
