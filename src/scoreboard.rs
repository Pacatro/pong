use bevy::prelude::*;

use crate::{GameState, player::{Player1, Player2, Score}};

const SCOREBOARD_FONT_SIZE: f32 = 100.0;
const SCORE_COLOR: Color = Color::WHITE;
const SCORE_FONT: &str = "fonts/I-pixel-u.ttf";

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_scoreboard)
            .add_systems(Update, update_scoreboard.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Debug, Component)]
pub struct ScoreBoard;

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
        TextBundle {
            text: Text::from_section(
                "", 
                text_style.clone()
            )
                .with_justify(JustifyText::Center),
            
            style: Style::default(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        ScoreBoard,
    ));

}

fn update_scoreboard(
    score1_query: Query<&Score, (With<Player1>, Without<Player2>)>,
    score2_query: Query<&Score, (With<Player2>, Without<Player1>)>,
    mut query: Query<&mut Text, With<ScoreBoard>>,
) {
    let score1 = score1_query.single();
    let score2 = score2_query.single();

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{} - {}", score1.get_value().to_string(), score2.get_value().to_string());
    }
}
