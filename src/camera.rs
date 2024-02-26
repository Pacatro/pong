use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, query: Query<&Window>) {
    let window: &Window = query.single();

    commands.spawn((
        Camera2dBundle::default(),
        Collider::cuboid(window.width()/2.0, window.height()/2.0)
    ));
}