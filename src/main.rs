mod camera;
mod player;
mod ball;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use camera::CameraPlugin;
use player::PlayerPlugin;
use ball::BallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .run()
}
