pub mod editing;
mod playing;

use bevy::prelude::*;

use crate::input::{PlayerInput, handle_input};

use editing::EditingPlugin;
use playing::PlayingPlugin;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(PlayingPlugin)
            .add_plugins(EditingPlugin)
            .add_systems(PostStartup, init)
            .add_systems(Update, state_transition.after(handle_input));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum AppState {
    #[default]
    Setup,
    Playing,
    Editing,
}

fn init(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn state_transition(
    inputs: Res<PlayerInput>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if inputs.change_mode_pressed() {
        match state.get() {
            AppState::Playing => next_state.set(AppState::Editing),
            AppState::Editing => next_state.set(AppState::Playing),
            AppState::Setup => next_state.set(AppState::Playing),
        }
    }
}
