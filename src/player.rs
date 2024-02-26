use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;

const PLAYER_X_LENGTH: f32 = 20.0;
const PLAYER_Y_LENGTH: f32 = 100.0;
const PLAYER1_TRANSLATION: Vec3 = Vec3::new(-600.0, 0.0, 0.0);
const PLAYER2_TRANSLATION: Vec3 = Vec3::new(600.0, 0.0, 0.0);

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_players)
            .add_systems(Update, (player_movement, player_info));
    }
}

#[derive(Debug, Component)]
struct Player;

fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(PLAYER_X_LENGTH, PLAYER_Y_LENGTH, 0.))),
            material: materials.add(Color::rgb(255.0, 87.0, 51.0)),
            transform: Transform::from_translation(PLAYER1_TRANSLATION),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PLAYER_X_LENGTH/2.0, PLAYER_Y_LENGTH/2.0),
        KinematicCharacterController::default(),
        Player
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(PLAYER_X_LENGTH, PLAYER_Y_LENGTH, 0.))),
            material: materials.add(Color::rgb(255.0, 87.0, 51.0)),
            transform: Transform::from_translation(PLAYER2_TRANSLATION),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PLAYER_X_LENGTH/2.0, PLAYER_Y_LENGTH/2.0),
        KinematicCharacterController::default(),
        Player
    ));
}

fn player_movement(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut controller in query.iter_mut() {
        let mut movement = Vec2::ZERO;
    
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            movement.y += 10.0; // Move right
        } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            movement.y -= 10.0; // Move left
        }
    
        controller.translation = Some(movement);
    }
}

fn player_info(controllers: Query<(Entity, &KinematicCharacterControllerOutput), With<Player>>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?}: {:?}", entity, output.effective_translation);
    }
}