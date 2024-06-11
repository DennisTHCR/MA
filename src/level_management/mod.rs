use bevy::{prelude::*, sprite::Mesh2dHandle};
use crate::utilities::assets::{ColorResource, init_resources};

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_level.after(init_resources));
    }
}

fn setup_level(mut commands: Commands, materials: Res<ColorResource>, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh: Mesh2dHandle = meshes.add(Rectangle::new(200., 10.)).into();
    commands.spawn(ColorMesh2dBundle{
        mesh,
        material: materials.0[0].0.clone(),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}