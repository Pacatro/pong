use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;

use crate::{ball::Ball, map::ScoreLimit};
// use std::env;

const PLAYER_X_LENGTH: f32 = 20.0;
const PLAYER_Y_LENGTH: f32 = 100.0;
const PLAYERS_COLOR: Color = Color::rgb(255.0, 255.0, 255.0);
const PLAYER1_TRANSLATION: Vec3 = Vec3::new(-600.0, 0.0, 0.0);
const PLAYER2_TRANSLATION: Vec3 = Vec3::new(600.0, 0.0, 0.0);

// fn get_player_velocity() -> f32 {
//     if env::consts::OS == "windows" { 5.0 } else { 15.0 }
// }

const PLAYERS_VELOCITY: f32 = 800.0;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_players)
            .add_systems(Update, (player_movement, increase_points));
    }
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Score {
    value: u32
}

impl Score {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn increase(&mut self, value: u32) {
        self.value += value;
    }
}

fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(PLAYER_X_LENGTH, PLAYER_Y_LENGTH, 0.))),
            material: materials.add(PLAYERS_COLOR),
            transform: Transform::from_translation(PLAYER1_TRANSLATION),
            ..default()
        },
        Score::new(),
        Player
    ))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(PLAYER_X_LENGTH/2.0, PLAYER_Y_LENGTH/2.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(0.0))
        .insert(Velocity::zero())
        .insert(Ccd::enabled())
        .insert(KinematicCharacterController::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(PLAYER_X_LENGTH, PLAYER_Y_LENGTH, 0.))),
            material: materials.add(PLAYERS_COLOR),
            transform: Transform::from_translation(PLAYER2_TRANSLATION),
            ..default()
        },
        Score::new(),
        Player
    ))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(PLAYER_X_LENGTH/2.0, PLAYER_Y_LENGTH/2.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(0.0))
        .insert(Ccd::enabled())
        .insert(KinematicCharacterController::default());
}

fn player_movement(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for mut controller in query.iter_mut() {
        let mut movement: Vec2 = Vec2::ZERO;
    
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            movement.y += PLAYERS_VELOCITY * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            movement.y -= PLAYERS_VELOCITY * time.delta_seconds();
        }
    
        controller.translation = Some(movement);
    }
}

fn increase_points(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    score_limit_query: Query<Entity, With<ScoreLimit>>,
    mut player_query: Query<&mut Score, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    for ball in ball_query.iter() {
        for score_limit in score_limit_query.iter() {
            for mut score in player_query.iter_mut() {
                if rapier_context.intersection_pair(ball, score_limit).is_some() {
                    score.increase(1);
                    commands.entity(ball).despawn_recursive();
                    println!("{score:?}")
                }
            }
        }
    }
}

// fn player_info(controllers: Query<(Entity, &KinematicCharacterControllerOutput), With<Player>>) {
//     for (entity, output) in controllers.iter() {
//         println!("Entity {:?}: {:?}", entity, output.effective_translation);
//     }
// }