use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input)
            .insert_resource(PlayerInput::default());
    }
}

fn handle_input(
    kb: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_input: ResMut<PlayerInput>,
) {
    player_input.jump = kb.just_pressed(KeyCode::Space)
        || kb.just_pressed(KeyCode::ArrowUp)
        || kb.just_pressed(KeyCode::KeyW);
    player_input.up =
        kb.pressed(KeyCode::Space) || kb.pressed(KeyCode::ArrowUp) || kb.pressed(KeyCode::KeyW);
    player_input.crouch = kb.just_pressed(KeyCode::ShiftLeft)
        || kb.just_pressed(KeyCode::ShiftRight)
        || kb.just_pressed(KeyCode::ArrowDown)
        || kb.just_pressed(KeyCode::KeyS);
    player_input.down = kb.pressed(KeyCode::ShiftLeft)
        || kb.pressed(KeyCode::ShiftRight)
        || kb.pressed(KeyCode::ArrowDown)
        || kb.pressed(KeyCode::KeyS);
    player_input.left = kb.pressed(KeyCode::ArrowLeft) || kb.pressed(KeyCode::KeyA);
    player_input.right = kb.pressed(KeyCode::ArrowRight) || kb.pressed(KeyCode::KeyD);
    player_input.change_mode = kb.just_pressed(KeyCode::KeyG);
    player_input.left_clicked = mouse.just_pressed(MouseButton::Left);
    player_input.left_click_held = mouse.pressed(MouseButton::Left);
    player_input.right_clicked = mouse.just_pressed(MouseButton::Right);
    player_input.right_click_held = mouse.pressed(MouseButton::Right);
}

#[allow(unused)]
#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerInput {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    jump: bool,
    crouch: bool,
    change_mode: bool,
    right_clicked: bool,
    right_click_held: bool,
    left_clicked: bool,
    left_click_held: bool,
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

    pub fn camera_vector(self) -> Vec2 {
        let mut out = Vec2::ZERO;
        if self.left {
            out += Vec2::new(-1., 0.);
        }
        if self.right {
            out += Vec2::new(1., 0.);
        }
        if self.up {
            out += Vec2::new(0., 1.)
        }
        if self.down {
            out += Vec2::new(0., -1.)
        }
        out
    }

    pub fn change_mode_pressed(self) -> bool {
        self.change_mode
    }

    pub fn right_clicked(self) -> bool {
        self.right_clicked
    }

    pub fn left_clicked(self) -> bool {
        self.left_clicked
    }
}
