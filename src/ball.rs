use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::player::Player;

const BALL_RADIUS: f32 = 25.0;
const INITIAL_BALL_VELOCITY: f32 = 550.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ball)
            .add_systems(PostStartup, move_ball)
            .add_systems(Update, increase_ball_velocity);
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
        .insert(Restitution::coefficient(1.0))
        .insert(ExternalImpulse::default())
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
    query_player: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (ball, mut velocity) = query_ball.single_mut();
    
    let mut velocity_x: f32 = velocity.linvel.x;
    let mut velocity_y: f32 = velocity.linvel.y;

    for player in query_player.iter() {
        if rapier_context.contact_pair(player, ball).is_some() {
            velocity_x += 100.0;
            velocity_y += 100.0;
            velocity.linvel = Vec2::new(velocity_x, velocity_y);
        }
    }

    println!("{velocity:?}")
}
