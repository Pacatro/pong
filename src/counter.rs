use bevy::prelude::*;

use crate::GameState;

const FONT_SIZE: f32 = 100.0;
const COUNTER_COLOR: Color = Color::WHITE;
const COUNTER_FONT: &str = "fonts/I-pixel-u.ttf";
const INITIAL_COUNTER: i32 = 3;

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CountdownTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .add_systems(OnEnter(GameState::Counter), spawn_counter)
            .add_systems(OnExit(GameState::Counter), despawn_counter)
            .add_systems(
                Update,
                (update_scoreboard, count_down).chain().run_if(in_state(GameState::Counter))
            );
    }
}

#[derive(Debug, Component)]
pub struct Counter;

#[derive(Debug, Component)]
struct CounterTimer(i32);

#[derive(Debug, Resource)]
struct CountdownTimer(Timer);

fn count_down(
    mut timer: ResMut<CountdownTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut query_counter: Query<&mut CounterTimer, With<Counter>>,
    time: Res<Time>,
) {
    let mut counter = query_counter.single_mut();

    if counter.0 > 0 {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            counter.0 -= 1;
            timer.0.reset();
        }
    } else {
        game_state.set(GameState::InGame);
        counter.0 = 3;
    }
}

fn update_scoreboard(
    query_counter: Query<&CounterTimer, With<Counter>>,
    mut counter_text_query: Query<&mut Text, With<Counter>>,
) {
    let counter = query_counter.single();

    if counter.0 > 0 {
        let mut counter_text = counter_text_query.single_mut();
        counter_text.sections[0].value = counter.0.to_string(); 
    }
}

fn despawn_counter(
    mut commands: Commands, 
    query: Query<Entity, With<Counter>>
) {
    let counter = query.single();
    commands.entity(counter).despawn_recursive();
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
    };

    commands.spawn((
        TextBundle::from_section(
            "",
            text_style.clone(),
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(5.0),
            ..default()
        }),
        CounterTimer(INITIAL_COUNTER),
        Counter
    ));
}
