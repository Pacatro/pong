mod camera;
mod player;
mod ball;
mod map;
mod scoreboard;
mod pause;

use bevy::{prelude::*, window::EnabledButtons};
use bevy_rapier2d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use map::MapPlugin;
use player::PlayersPlugin;
use ball::BallPlugin;
use scoreboard::ScoreBoardPlugin;
use pause::PausePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Pause,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        minimize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }
        ))
        .init_state::<GameState>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) //* FOR DEBUGGING PHYSICS
        // .add_plugins(WorldInspectorPlugin::new()) //* FOR DEBUGGING INSPECTOR
        .add_plugins(CameraPlugin)
        .add_plugins(PausePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(ScoreBoardPlugin)
        .run();
}