use crate::game::GameState;
use bevy::prelude::*;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_system(hide.in_schedule(OnExit(GameState::Terminal)))
            .add_system(show.in_schedule(OnEnter(GameState::Terminal)))
            .add_system(input.in_set(OnUpdate(GameState::Terminal)))
            .add_system(text_update_system.in_set(OnUpdate(GameState::Terminal)));
    }
}

#[derive(Component, Default)]
struct Terminal {
    current: String,
    history: Vec<String>,
}

#[derive(Component)]
struct TerminalText;

#[derive(Component)]
struct CurrentText;

#[derive(Component)]
struct HistoryText;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Terminal::default());

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "> ",
                TextStyle {
                    font: asset_server.load("fonts/fira-code/regular.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/fira-code/medium.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            }),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        CurrentText,
        TerminalText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "History:\n",
                TextStyle {
                    font: asset_server.load("fonts/fira-code/bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/fira-code/medium.ttf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]),
        HistoryText,
        TerminalText,
    ));
}

fn hide(mut query: Query<&mut Visibility, With<TerminalText>>) {
    for mut visibilty in &mut query {
        *visibilty = Visibility::Hidden;
    }
}

fn show(mut query: Query<&mut Visibility, With<TerminalText>>) {
    for mut visibilty in &mut query {
        *visibilty = Visibility::Inherited;
    }
}

// TODO: Fix so characters show up on screen as you type.
fn input(
    mut event_reader: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut terminal: Query<&mut Terminal>,
    mut text: Query<&mut Text, With<CurrentText>>,
) {
    let terminal = &mut terminal.single_mut();
    let text = &mut text.single_mut();

    text.sections[1].value = (*terminal).current.to_string();

    for event in event_reader.iter() {
        println!("Got char: '{}'", event.char);

        if event.char == char::from(0x08) {
            terminal.current.pop();
        } else {
            terminal.current.push(event.char);
        }
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", &terminal.current);
        let string = terminal.current.clone();
        terminal.history.push(string);
        terminal.current.clear();
    }
}

// TODO: Fix commands so that they show up correctly.
fn text_update_system(mut text: Query<&mut Text, With<HistoryText>>, terminal: Query<&Terminal>) {
    let terminal = terminal.single();

    for mut text in &mut text {
        text.sections[1].value = format!("{:?}", terminal.history);
    }
}
