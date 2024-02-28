mod camera;
mod player;
mod ball;
mod map;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use map::MapPlugin;
use player::PlayersPlugin;
use ball::BallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(BallPlugin)
        .run();
}
