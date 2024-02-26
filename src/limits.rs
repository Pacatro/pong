use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LimitsPlugin;

impl Plugin for LimitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_limits);
    }
}

#[derive(Debug, Component)]
struct Limit;

fn set_limits(
    mut commands: Commands,
    query: Query<&Window>,
) {
    let window: &Window = query.single();
    
    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, window.height(), 0.0))),        
        Collider::cuboid(window.width()/2.0, window.height()/2.0),
        Limit
    ));

    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -window.height(), 0.0))),        
        Collider::cuboid(window.width()/2.0, window.height()/2.0),
        Limit
    ));
}