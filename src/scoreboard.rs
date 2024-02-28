use bevy::prelude::*;

use crate::player::{Player, Score};

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, show_scoreboard);
    }
}

#[derive(Debug, Component)]
pub struct ScoreBoard;

fn show_scoreboard(
    mut commands: Commands,
    query: Query<&Score, With<Player>>,
    asset_server: Res<AssetServer>
) {

    for score in query.iter() {
        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                score.get_value().to_string(),
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/Symtext.ttf"),
                    font_size: 100.0,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }),
            ScoreBoard,
        ));
    }
}

