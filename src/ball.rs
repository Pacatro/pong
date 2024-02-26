use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;

const BALL_RADIUS: f32 = 10.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ball);
    }
}

#[derive(Debug, Component)]
struct Ball;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS))),
            material: materials.add(Color::rgb(255.0, 87.0, 51.0)),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(BALL_RADIUS),
        KinematicCharacterController::default(),
        Ball
    ));
}