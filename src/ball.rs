use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::player::Player;

const BALL_RADIUS: f32 = 20.0;
const INITIAL_BALL_VELOCITY: f32 = 400.0;
const DESPAWN_DISTANCE: f32 = 2000.0;
const INCREASE_FACTOR: f32 = 2.0;
const INCREASE_PERCENTAGE_FACTOR: f32 = 0.01;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ball)
            .add_systems(PostStartup, move_ball)
            .add_systems(Update, (increase_ball_velocity, despawn_ball));
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
        Ball
    ))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::ball(BALL_RADIUS))
        .insert(GravityScale(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(2.0))
        .insert(Ccd::enabled());
}

fn move_ball(
    mut query: Query<&mut Velocity, With<Ball>>
) {
    let mut velocity = query.single_mut();

    let rand_x: f32 = if rand::thread_rng().gen_range(-1..=1) == 0 { 1.0 } else { -1.0 };
    let rand_y: f32 = if rand::thread_rng().gen_range(-1..=1) == 0 { 1.0 } else { -1.0 };

    velocity.linvel = Vec2::new(rand_x, rand_y).normalize() * INITIAL_BALL_VELOCITY;

}

fn increase_ball_velocity(
    mut query_ball: Query<(Entity, &mut Velocity), With<Ball>>,
    mut query_player: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    for (ball, mut velocity) in query_ball.iter_mut() {
        for player in query_player.iter_mut() {
            if rapier_context.contact_pair(player, ball).is_some() {
                velocity.linvel *= 1.0 + INCREASE_FACTOR * INCREASE_PERCENTAGE_FACTOR;
            }
        }
    }
}

fn despawn_ball(
    mut commands: Commands, 
    ball_query: Query<(Entity, &Transform), With<Ball>>,
) {
    for (ball, transform) in ball_query.iter() {
        if transform.translation.distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            commands.entity(ball).despawn_recursive();
        }
    }
}
