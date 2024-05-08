use bevy::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSettings};

use crate::{GameModeState, GameState};


const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct LoginMenuPlugin;

impl Plugin for LoginMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .add_systems(OnEnter(GameState::LoginForm), spawn_login_menu.run_if(in_state(GameModeState::Online)));
    }
}

fn spawn_login_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: BORDER_COLOR_ACTIVE.into(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                },
                TextInputBundle::default()
                    .with_value("password")
                    .with_text_style(TextStyle {
                        font_size: 40.,
                        color: TEXT_COLOR,
                        ..default()
                    })
                    .with_settings(TextInputSettings {
                        retain_on_submit: true,
                    }),
            ));
        });
}