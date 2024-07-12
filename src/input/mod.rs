use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input)
            .insert_resource(PlayerInput::default());
    }
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.jump = input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyW);
    player_input.crouch = input.just_pressed(KeyCode::ShiftLeft) || input.just_pressed(KeyCode::ShiftRight) || input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::KeyS);
    player_input.left = input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA);
    player_input.right = input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD)
}

#[allow(unused)]
#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerInput {
    left: bool,
    right: bool,
    jump: bool,
    crouch: bool,
}

#[allow(unused)]
impl PlayerInput {
    pub fn jump_pressed(self) -> bool {
        self.jump
    }

    pub fn crouch_pressed(self) -> bool {
        self.crouch
    }

    pub fn direction_vector(self) -> Vec2 {
        let mut out = Vec2::ZERO;
        if self.left {
            out += Vec2::new(-1., 0.);
        }
        if self.right {
            out += Vec2::new(1., 0.);
        }
        out
    }
}
