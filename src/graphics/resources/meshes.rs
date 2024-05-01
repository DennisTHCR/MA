use bevy::{prelude::*, sprite::Mesh2dHandle};

pub struct MeshContainerPlugin;

impl Plugin for MeshContainerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(MeshContainer {
        square: Mesh2dHandle(meshes.add(Rectangle::default()))
    });
}

#[derive(Resource)]
pub struct MeshContainer {
    pub square: Mesh2dHandle
}