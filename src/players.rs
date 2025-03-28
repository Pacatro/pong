use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::{
    ball::Ball,
    map::{ScoreLimit1, ScoreLimit2},
    GameState,
};

const PLAYER_X_LENGTH: f32 = 20.0;
const PLAYER_Y_LENGTH: f32 = 100.0;
const PLAYERS_VELOCITY: f32 = 800.0;
const PLAYERS_COLOR: Color = Color::WHITE;
const PLAYER1_TRANSLATION: Vec3 = Vec3::new(-600.0, 0.0, 0.0);
const PLAYER2_TRANSLATION: Vec3 = Vec3::new(600.0, 0.0, 0.0);

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::MainMenu), spawn_players)
            .add_systems(
                Update,
                (player_movement, increase_points).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Debug, Component)]
pub struct Player1;

#[derive(Debug, Component)]
pub struct Player2;
#[derive(Debug, Component)]
pub struct Score {
    value: u32,
}

impl Score {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn add(&mut self, value: u32) {
        self.value += value;
    }
}

fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(PLAYER_X_LENGTH, PLAYER_Y_LENGTH, 0.))),
            material: materials.add(PLAYERS_COLOR),
            transform: Transform::from_translation(PLAYER1_TRANSLATION),
            ..default()
        },
        Score::new(),
        Player1,
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(PLAYER_X_LENGTH / 2.0, PLAYER_Y_LENGTH / 2.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(0.0),
        Velocity::zero(),
        KinematicCharacterController::default(),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(PLAYER_X_LENGTH, PLAYER_Y_LENGTH, 0.))),
            material: materials.add(PLAYERS_COLOR),
            transform: Transform::from_translation(PLAYER2_TRANSLATION),
            ..default()
        },
        Score::new(),
        Player2,
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(PLAYER_X_LENGTH / 2.0, PLAYER_Y_LENGTH / 2.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(0.0),
        Velocity::zero(),
        KinematicCharacterController::default(),
    ));
}

fn player_movement(
    mut player1_query: Query<&mut KinematicCharacterController, (With<Player1>, Without<Player2>)>,
    mut player2_query: Query<&mut KinematicCharacterController, (With<Player2>, Without<Player1>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player1_controller = player1_query.single_mut();
    let mut player2_controller = player2_query.single_mut();

    let mut player1_movement: Vec2 = Vec2::ZERO;
    let mut player2_movement: Vec2 = Vec2::ZERO;

    // Player 1 movement
    if keyboard_input.pressed(KeyCode::KeyW) {
        player1_movement.y += PLAYERS_VELOCITY * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        player1_movement.y -= PLAYERS_VELOCITY * time.delta_seconds();
    }

    // Player 2 movement
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        player2_movement.y += PLAYERS_VELOCITY * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        player2_movement.y -= PLAYERS_VELOCITY * time.delta_seconds();
    }

    player1_controller.translation = Some(player1_movement);
    player2_controller.translation = Some(player2_movement);
}

fn increase_points(
    mut ball_query: Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
    mut player1_query: Query<&mut Score, (With<Player1>, Without<Player2>)>,
    mut player2_query: Query<&mut Score, (With<Player2>, Without<Player1>)>,
    score_limit1_query: Query<Entity, With<ScoreLimit1>>,
    score_limit2_query: Query<Entity, With<ScoreLimit2>>,
    rapier_context: Res<RapierContext>,
) {
    let (ball, mut ball_transform, mut ball_velocity) = ball_query.single_mut();

    let mut score1 = player1_query.single_mut();
    let mut score2 = player2_query.single_mut();

    let score_limit1 = score_limit1_query.single();
    let score_limit2 = score_limit2_query.single();

    if rapier_context
        .intersection_pair(ball, score_limit1)
        .is_some()
    {
        score2.add(1);
        ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
        ball_velocity.linvel = Ball::get_init_velocity();
    } else if rapier_context
        .intersection_pair(ball, score_limit2)
        .is_some()
    {
        score1.add(1);
        ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
        ball_velocity.linvel = Ball::get_init_velocity();
    }
}
