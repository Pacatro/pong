use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{GameState, player::{Player1, Player2}};

const BALL_RADIUS: f32 = 20.0;
const INITIAL_BALL_VELOCITY: f32 = 400.0;
const INCREASE_FACTOR: f32 = 3.0;
const INCREASE_PERCENTAGE_FACTOR: f32 = 0.01;
const BALL_COLOR: Color = Color::WHITE;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ball)
            .add_systems(PostStartup, move_ball)
            .add_systems(Update, increase_ball_velocity.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Debug, Component)]
pub struct Ball;

impl Ball {
    pub fn get_init_velocity() -> Vec2 {
        let rand_x: f32 = if rand::thread_rng().gen_bool(0.5) { 1.0 } else { -1.0 };
        let rand_y: f32 = if rand::thread_rng().gen_bool(0.5) { 1.0 } else { -1.0 };
        Vec2::new(rand_x, rand_y).normalize() * INITIAL_BALL_VELOCITY
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS))),
            material: materials.add(BALL_COLOR),
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
    mut query: Query<&mut Velocity, With<Ball>>,
) {
    let mut velocity = query.single_mut();
    velocity.linvel = Ball::get_init_velocity();
}

fn increase_ball_velocity(
    mut query_ball: Query<(Entity, &mut Velocity), With<Ball>>,
    query_player1: Query<Entity, With<Player1>>,
    query_player2: Query<Entity, With<Player2>>,
    rapier_context: Res<RapierContext>,
) {
    let (ball, mut velocity) = query_ball.single_mut();
    let player1 = query_player1.single();
    let player2 = query_player2.single();
    
    if rapier_context.contact_pair(player1, ball).is_some()
    || rapier_context.contact_pair(player2, ball).is_some() {
        velocity.linvel *= 1.0 + INCREASE_FACTOR * INCREASE_PERCENTAGE_FACTOR;
    }
}