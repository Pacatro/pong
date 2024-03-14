use bevy::prelude::*;

use crate::GameState;

const FONT_SIZE: f32 = 100.0;
const COUNTER_COLOR: Color = Color::WHITE;
const COUNTER_FONT: &str = "fonts/I-pixel-u.ttf";

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CounterTimer(3))
            .insert_resource(CountdownTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .add_systems(OnEnter(GameState::Counter), spawn_counter)
            .add_systems(Update, (update_scoreboard, count_down).chain().run_if(in_state(GameState::Counter)));
    }
}

#[derive(Debug, Component)]
pub struct Counter;

#[derive(Debug, Resource)]
struct CounterTimer(i32);

#[derive(Debug, Resource)]
struct CountdownTimer(Timer);

fn count_down(
    mut counter: ResMut<CounterTimer>,
    mut timer: ResMut<CountdownTimer>,
    time: Res<Time>,
) {
    if counter.0 > 0 {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            println!("{}", counter.0);
            counter.0 -= 1;
            timer.0.reset();
        }
    }
}

fn update_scoreboard(
    counter: ResMut<CounterTimer>,
    mut counter_text_query: Query<&mut Text, With<Counter>>,
) {
    let mut counter_text = counter_text_query.single_mut();
    counter_text.sections[0].value = format!("{}", counter.0.to_string()); 
}

fn spawn_counter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(COUNTER_FONT);

    let text_style = TextStyle {
        font_size: FONT_SIZE,
        font,
        color: COUNTER_COLOR,
        ..default()
    };

    commands.spawn((
        TextBundle::from_section(
            "",
            text_style.clone(),
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                justify_content: JustifyContent::Center,
                ..default()
            }),

        Counter
    ));
}

