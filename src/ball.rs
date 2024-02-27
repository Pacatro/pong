use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::player::Player;

const BALL_RADIUS: f32 = 10.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ball)
            .add_systems(Update, (move_ball, show_ball_info));
    }
}

#[derive(Debug, Component)]
pub struct Ball;

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
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::ball(BALL_RADIUS),
        ExternalForce::default(),
        Ball
    ));
}

fn move_ball(
    mut query: Query<&mut ExternalForce, With<Ball>>,
    query_ball: Query<Entity, With<Ball>>,
    query_player: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    todo!()
}

fn show_ball_info(
    rapier_context: Res<RapierContext>,
    query_ball: Query<Entity, With<Ball>>,
    query_player: Query<Entity, With<Player>>
) {
    for player in query_player.iter() {
        for ball in query_ball.iter() {
            if rapier_context.contact_pair(player, ball).is_some() {
                println!("SAN CHOCAO")
            }
        }
    }
}
