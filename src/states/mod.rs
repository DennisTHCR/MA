pub mod editing;
mod playing;

use bevy::prelude::*;

use crate::input::{handle_input, PlayerInput};

use editing::EditingPlugin;
use playing::PlayingPlugin;

/// The Plugin containing everything related to the [AppState] enum
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

/// The enum that defines all possible states of this app
#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum AppState {
    #[default]
    Setup,
    Playing,
    Editing,
}

/// This System is run on startup and sets the initial [AppState]
fn init(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

/// This System updates the current [AppState], if the G button was just pressed down
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
