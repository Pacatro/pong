mod ball;
mod camera;
mod counter;
mod main_menu;
mod map;
mod pause;
mod players;
mod scoreboard;

use bevy::{prelude::*, window::EnabledButtons};
use bevy_rapier2d::prelude::*;

use ball::BallPlugin;
use camera::CameraPlugin;
use counter::CounterPlugin;
use main_menu::MainMenuPlugin;
use map::MapPlugin;
use pause::PausePlugin;
use players::PlayersPlugin;
use scoreboard::ScoreBoardPlugin;

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
    // TODO
    // Online,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
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
        .run();
}
