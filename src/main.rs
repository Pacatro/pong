mod camera;
mod player;
mod ball;
mod limits;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use camera::CameraPlugin;
use limits::LimitsPlugin;
use player::PlayersPlugin;
use ball::BallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(LimitsPlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(BallPlugin)
        .run()
}
