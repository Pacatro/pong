use bevy::{app::AppExit, prelude::*};

use crate::{GameState, GameModeState};

const TITLE_COLOR: Color = Color::WHITE;
const TEXT_COLOR: Color = Color::BLACK;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Multiplayer,
    Quit,
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut gamemode_state: ResMut<NextState<GameModeState>>,
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {

    let main_menu = main_menu_query.single();

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                
                MenuButtonAction::Play => {
                    game_state.set(GameState::Counter);

                    if gamemode_state.is_changed() {
                        gamemode_state.set(GameModeState::Offline);
                    }
                    
                    commands.entity(main_menu).despawn_recursive();
                }
                
                // TODO
                MenuButtonAction::Multiplayer => gamemode_state.set(GameModeState::Online),
            }
        }
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(260.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
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
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "PONG",
                            TextStyle {
                                font_size: 80.0,
                                color: TITLE_COLOR,
                                font: asset_server.load("fonts/I-pixel-u.ttf"),
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
                                style: button_style.clone(),
                                image: UiImage::default(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("PLAY", button_text_style.clone()));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                image: UiImage::default(),
                                ..default()
                            },
                            MenuButtonAction::Multiplayer,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "MULTIPAYER", // TODO
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                image: UiImage::default(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("QUIT", button_text_style));
                        });
                });
        });
}