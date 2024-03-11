use bevy::prelude::*;

use crate::GameState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const TEXT_COLOR: Color = Color::WHITE;
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, set_pause.run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::Pause), spawn_pause_menu)
            .add_systems(Update, set_in_game.run_if(in_state(GameState::Pause)))
            .add_systems(OnExit(GameState::Pause), delete_pause_menu);
    }
}

#[derive(Debug, Component)]
pub struct PauseButton;

fn set_pause(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
    }
}

fn set_in_game(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        time.unpause();
        game_state.set(GameState::InGame);
    }
}

// TODO: Change this to a normal text
fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time<Virtual>>
) {
    time.pause();

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        PauseButton
    ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "PRESS SPACE TO CONTINUE",
                        TextStyle {
                            font: asset_server.load("fonts/I-pixel-u.ttf"),
                            font_size: 40.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
        });
}

fn delete_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<Button>>
) {
    let button = query.single();
    commands.entity(button).despawn_recursive()
}