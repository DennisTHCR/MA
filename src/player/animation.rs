use bevy::prelude::*;

use crate::{
    input::PlayerInput,
    states::AppState,
    utilities::assets::{PlayerAnimationIndices, PlayerAnimationMap, PlayerAnimationState},
};

use super::PlayerMarker;

/// A Plugin for managing the player animation
pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_player.run_if(in_state(AppState::Playing)));
    }
}

/// A System that changes the players animation, depending on state
fn animate_player(
    player_input: Res<PlayerInput>,
    player_animation_map: Res<PlayerAnimationMap>,
    mut player_animation_indices: ResMut<PlayerAnimationIndices>,
    mut query: Query<(&mut TextureAtlas, &mut Handle<Image>), With<PlayerMarker>>,
    mut player_animation_state_res: ResMut<PlayerAnimationState>,
) {
    let mut player_animation_state = PlayerAnimationState::IDLE;
    if player_input.direction_vector().x.abs() >= 0.1 {
        player_animation_state = PlayerAnimationState::RUN;
    }
    if player_animation_state == *player_animation_state_res {
        return;
    }
    *player_animation_state_res = player_animation_state;
    let indices = match player_animation_state {
        PlayerAnimationState::FALL => 1,
        PlayerAnimationState::IDLE => 11,
        PlayerAnimationState::JUMP => 1,
        PlayerAnimationState::RUN => 12,
    };
    player_animation_indices.last = indices - 1;
    let (mut atlas, mut image) = query.single_mut();
    atlas.index = 0;
    atlas.layout = player_animation_map
        .0
        .get(&player_animation_state)
        .unwrap()
        .texture_atlas_layout
        .clone();
    *image = player_animation_map
        .0
        .get(&player_animation_state)
        .unwrap()
        .texture
        .clone();
}
