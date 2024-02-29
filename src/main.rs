mod camera;
mod player;
mod ball;
mod map;
mod scoreboard;

use bevy::{prelude::*, window::EnabledButtons};
use bevy_rapier2d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use map::MapPlugin;
use player::PlayersPlugin;
use ball::BallPlugin;
use scoreboard::ScoreBoardPlugin;

fn main() {
    App::new()
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
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(ScoreBoardPlugin)
        .run();
}