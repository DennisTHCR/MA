use bevy::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_resources);
    }
}

pub fn init_resources(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material_white = materials.add(ColorMaterial::from(Color::WHITE));
    commands.insert_resource(ColorResource([(
        material_white,
        "WHITE".to_string(),
        Color::WHITE,
    )]));
}

#[derive(Resource)]
pub struct ColorResource(pub [(Handle<ColorMaterial>, String, Color); 1]);
