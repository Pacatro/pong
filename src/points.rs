use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ball::Ball;
use crate::player::{Player, Points};
use crate::map::Goal;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, increase_points);
    }
}

fn increase_points(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    goal_query: Query<Entity, With<Goal>>,
    mut player_query: Query<&mut Points, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    
    for ball in ball_query.iter() {
        for goal in goal_query.iter() {
            for mut points in player_query.iter_mut() {
                if rapier_context.intersection_pair(ball, goal).is_some() {
                    points.increase(1);
                    commands.entity(ball).despawn_recursive();
                    println!("{points:?}")
                }
            }
        }
    }
}