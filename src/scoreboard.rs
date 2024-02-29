use bevy::prelude::*;

use crate::player::{Player1, Player2, Score};

const SCOREBOARD_FONT_SIZE: f32 = 100.0;
const SCORE_COLOR: Color = Color::rgb(255.0, 255.0, 255.0);
const SCORE_FONT: &str = "fonts/I-pixel-u.ttf";

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (show_scoreboard));
    }
}

#[derive(Debug, Component)]
pub struct ScoreBoard;

fn show_scoreboard(
    mut commands: Commands,
    score1_query: Query<&Score, (With<Player1>, Without<Player2>)>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "0",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    font: asset_server.load(SCORE_FONT),
                    color: SCORE_COLOR,
                    ..default()
                }
            ),
            TextSection::new(
                "1",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    font: asset_server.load(SCORE_FONT),
                    color: SCORE_COLOR,
                    ..default()
                }
            )
        ])
            .with_text_justify(JustifyText::Center),
        ScoreBoard,
    ));
}

// fn update_scoreboard(
//     score1_query: Query<&Score, (With<Player1>, Without<Player2>)>,
//     score2_query: Query<&Score, (With<Player2>, Without<Player1>)>,
//     mut query: Query<&mut Text, With<ScoreBoard>>,
// ) {
//     let mut text = query.single_mut();
//     let score1 = score1_query.single();
//     let score2 = score2_query.single();

    
// }
