mod camera;
mod players;
mod ball;
mod map;
mod scoreboard;
mod pause;
mod main_menu;
mod counter;
mod client;

use bevy::{prelude::*, window::EnabledButtons};
use bevy_rapier2d::prelude::*;

use camera::CameraPlugin;
use counter::CounterPlugin;
use main_menu::MainMenuPlugin;
use map::MapPlugin;
use players::PlayersPlugin;
use ball::BallPlugin;
use scoreboard::ScoreBoardPlugin;
use pause::PausePlugin;
use client::ClientPlugin;

// TODO: MADE SERVER
// use server; 

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Counter,
    InGame,
    Pause,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameModeState {
    #[default]
    Offline,
    Online, // TODO
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }
        ))
        .init_state::<GameState>()
        .init_state::<GameModeState>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) //* FOR DEBUGGING PHYSICS
        .add_plugins(MainMenuPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PausePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(ScoreBoardPlugin)
        .add_plugins(CounterPlugin)
        .add_plugins(ClientPlugin)
        .run();
}