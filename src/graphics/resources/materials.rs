use bevy::prelude::*;

pub struct MaterialContainerPlugin;

impl Plugin for MaterialContainerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(MaterialContainer {
        white: materials.add(Color::hex("FFFFFF").unwrap())
    });
}

#[derive(Resource)]
pub struct MaterialContainer {
    pub white: Handle<ColorMaterial>
}