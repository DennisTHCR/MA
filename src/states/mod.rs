use bevy::prelude::*;

use crate::input::PlayerInput;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Update, state_transition)
            .add_systems(OnExit(AppState::Playing), exit_playing)
            .add_systems(OnEnter(AppState::Editing), enter_editing);
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum AppState {
    #[default]
    Playing,
    Editing
}

fn state_transition(inputs: Res<PlayerInput>, state: Res<State<AppState>>, mut next_state: ResMut<NextState<AppState>>) {
    if inputs.change_mode_pressed() {
        match state.get() {
            AppState::Playing => next_state.set(AppState::Editing),
            AppState::Editing => next_state.set(AppState::Playing),
        }
    }
}

// TODO: Remove Camera Follow mode
// Remove player physics? idk
fn exit_playing() {

}

// TODO: Make Camera move through arrow keys / dragging or sth idk
fn enter_editing() {

}