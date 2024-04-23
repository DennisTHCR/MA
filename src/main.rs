mod graphics;

use bevy::prelude::*;
use graphics::GraphicsPlugin;
fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        GraphicsPlugin,
    ))
    .run();
}
