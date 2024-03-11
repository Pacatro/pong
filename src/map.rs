use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::*;

use crate::GameState;

const LINE_COLOR: Color = Color::WHITE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (set_limits, set_center_line));
    }
}

#[derive(Debug, Component)]
pub struct Limit;

#[derive(Debug, Component)]
pub struct ScoreLimit1;

#[derive(Debug, Component)]
pub struct ScoreLimit2;

fn set_limits(
    mut commands: Commands,
    query: Query<&Window>,
) {
    let window: &Window = query.single();
    
    // Up limit
    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, window.height(), 0.0))),
        Limit
    ))
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(window.width()/2.0, window.height()/2.0))
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0));

    // Bottom limit
    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -window.height(), 0.0))),        
        Limit
    ))
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(window.width()/2.0, window.height()/2.0))
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0));

    // Score limit 1
    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(-(window.width()/2.0), 0.0, 0.0))),        
        ScoreLimit1
    ))
        .insert(Sensor)
        .insert(Collider::cuboid(1.0, window.height()/2.0));

    // Score limit 2
    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(window.width()/2.0, 0.0, 0.0))),        
        ScoreLimit2
    ))
        .insert(Sensor)
        .insert(Collider::cuboid(1.0, window.height()/2.0));
}

fn set_center_line(
    mut commands: Commands,
    query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let window: &Window = query.single();
    
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid::new(1.0, window.height(), 0.0))),
            transform: Transform::from_translation(Vec3::ZERO),
            material: materials.add(LINE_COLOR),
            ..default()
        }
    );
}