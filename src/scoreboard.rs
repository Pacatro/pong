use bevy::prelude::*;

use crate::{GameState, player::{Player1, Player2, Score}};

const SCOREBOARD_FONT_SIZE: f32 = 100.0;
const SCORE_COLOR: Color = Color::WHITE;
const SCORE_FONT: &str = "fonts/I-pixel-u.ttf";
const DISTANCE_BETWEEN_SCOREBOARDS: f32 = 500.0;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_scoreboard)
            .add_systems(Update, update_scoreboard.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Debug, Component)]
pub struct ScoreBoardP1;

#[derive(Debug, Component)]
pub struct ScoreBoardP2;

fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(SCORE_FONT);

    let text_style = TextStyle {
        font_size: SCOREBOARD_FONT_SIZE,
        font,
        color: SCORE_COLOR,
        ..default()
    };

    commands.spawn((
        TextBundle::from_section(
            "",
            text_style.clone(),
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(DISTANCE_BETWEEN_SCOREBOARDS),
                ..default()
            }),

        ScoreBoardP1,
    ));

    commands.spawn((
        TextBundle::from_section(
            "",
            text_style.clone(),
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Px(DISTANCE_BETWEEN_SCOREBOARDS),
                ..default()
            }),

        ScoreBoardP2,
    ));

}

fn update_scoreboard(
    score1_query: Query<&Score, (With<Player1>, Without<Player2>)>,
    score2_query: Query<&Score, (With<Player2>, Without<Player1>)>,
    mut query_p1: Query<&mut Text, (With<ScoreBoardP1>, Without<ScoreBoardP2>)>,
    mut query_p2: Query<&mut Text, (With<ScoreBoardP2>, Without<ScoreBoardP1>)>,
) {
    let score1 = score1_query.single();
    let score2 = score2_query.single();

    let mut text_p1 = query_p1.single_mut();
    let mut text_p2 = query_p2.single_mut();

    text_p1.sections[0].value = format!("{}", score1.get_value().to_string());
    text_p2.sections[0].value = format!("{}", score2.get_value().to_string());
}