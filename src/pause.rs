use bevy::prelude::*;

use crate::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, set_pause)
            .add_systems(OnEnter(GameState::Pause), pause)
            .add_systems(Update, set_in_game.run_if(in_state(GameState::Pause)));
    }
}

fn set_pause(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
    }
}

fn set_in_game(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        time.unpause();
        game_state.set(GameState::InGame);
    }
}

fn pause(
    mut time: ResMut<Time<Virtual>>
) {
    time.pause();
}