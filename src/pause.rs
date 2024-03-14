use bevy::{app::AppExit, prelude::*};

use crate::GameState;

const TEXT_COLOR: Color = Color::WHITE;
const MENU_BACKGROUND_COLOR: Color = Color::rgb(0.01, 0.01, 0.01);

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, set_in_pause.run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::Pause), spawn_pause_menu)
            .add_systems(Update, (set_in_game, menu_action).run_if(in_state(GameState::Pause)))
            .add_systems(OnExit(GameState::Pause), delete_pause_menu);
    }
}

#[derive(Debug, Component)]
pub struct PauseMenu;

#[derive(Component)]
enum PauseButtonAction {
    Quit,
}

fn set_in_pause(
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

fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time<Virtual>>
) {
    time.pause();

    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 30.0,
        color: Color::BLACK,
        font: asset_server.load("fonts/I-pixel-u.ttf"),
        ..default()
    };

    commands
        .spawn((
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
            PauseMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    border_color: BorderColor(Color::WHITE),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "PRESS SPACE TO CONTINUE",
                            TextStyle {
                                font_size: 35.0,
                                font: asset_server.load("fonts/I-pixel-u.ttf"),
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                image: UiImage::default(),
                                ..default()
                            },
                            PauseButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("game_icons/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("QUIT", button_text_style));
                        });
                });
        });
}

fn delete_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenu>>
) {
    let menu = query.single();
    commands.entity(menu).despawn_recursive()
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &PauseButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PauseButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}