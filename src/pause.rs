use bevy::prelude::*;

use crate::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Pause), pause);
    }
}

fn check_pause(
    game_state: Res<State<GameState>>,
) {
    
}

fn pause() {
    println!("Pause")
}