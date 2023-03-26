use crate::game::GameState;
use bevy::prelude::*;

const USER_LEVEL_COMMANDS: [(&str, &str); 3] = [
    ("login", "(elevates permissions if access card is valid)"),
    ("clear", "(clears the screen)"),
    ("show", "<property>"),
];

const ADMIN_LEVEL_COMMANDS: [(&str, &str); 5] = [
    ("update", "<version>"),
    ("save", "<in-file> <out-file>"),
    ("show", "<property>"),
    ("clear", "(clears the screen)"),
    ("logout", "(deelevates permissions)"),
];

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AccessLevel>()
            .add_startup_system(startup)
            .add_systems((
                hide.in_schedule(OnExit(GameState::Terminal)),
                show.in_schedule(OnEnter(GameState::Terminal)),
                input.in_set(OnUpdate(GameState::Terminal)),
                text_update_system.in_set(OnUpdate(GameState::Terminal)),
                // elevate_privileges.in_schedule(OnEnter(AccessLevel::Admin)),
                // deelevate_privileges.in_schedule(OnEnter(AccessLevel::User)),
            ));
    }
}

enum Kind {
    Command,
    Clear,
    Error,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Error => write!(f, "error: "),
            _ => write!(f, ""),
        }
    }
}

enum Line {
    Show(Property),
    Login,
    Logout,
    Update(String),
    Save(String, String),
    Clear,
    Empty,
    Error(String),
}

enum Property {}

impl TryFrom<&str> for Property {
    type Error = String;

    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        Err("not implemented".to_string())
    }
}

impl From<&str> for Line {
    fn from(command: &str) -> Self {
        let command = command.split(" ").collect::<Vec<_>>();

        let first_command = match command.get(0) {
            Some(&command) => command,
            None => return Line::Empty,
        };

        match first_command {
            "clear" => Line::Clear,
            "login" => Line::Login,
            "logout" => Line::Logout,
            "show" => {
                let property = match command.get(1) {
                    Some(&property) => property,
                    None => return Line::Error(String::from("not enough arguments, expected 1")),
                };

                match Property::try_from(property) {
                    Ok(property) => Line::Show(property),
                    Err(error) => Line::Error(error),
                }
            }
            "update" => {
                let version = match command.get(1) {
                    Some(&property) => property,
                    None => return Line::Error(String::from("not enough arguments, expected 1")),
                };

                Line::Update(version.to_string())
            }
            "save" => {
                let in_file = match command.get(1) {
                    Some(&file) => file,
                    None => return Line::Error(String::from("not enough arguments, expected 2")),
                };

                let out_file = match command.get(1) {
                    Some(&file) => file,
                    None => return Line::Error(String::from("not enough arguments, expected 2")),
                };

                Line::Save(in_file.to_string(), out_file.to_string())
            }
            "" => Line::Empty,
            _ => Line::Error("bad command".to_string()),
        }
    }
}

struct Command {
    kind: Kind,
    line: String,
}

impl Command {
    fn query(access_level: &AccessLevel, line: &str) -> Command {
        if line.is_empty() {
            return match access_level {
                AccessLevel::User => Self {
                    kind: Kind::Command,
                    line: format!(
                        "These are the available commands: \n- {}",
                        USER_LEVEL_COMMANDS
                            .iter()
                            .map(|(command, description)| format!("{command} {description}"))
                            .collect::<Vec<_>>()
                            .join("\n- ")
                    ),
                },
                AccessLevel::Admin => Self {
                    kind: Kind::Command,
                    line: format!(
                        "These are the available commands. \n- {}",
                        ADMIN_LEVEL_COMMANDS
                            .iter()
                            .map(|(command, description)| format!("{command} {description}"))
                            .collect::<Vec<_>>()
                            .join("\n- ")
                    ),
                },
            };
        }

        let res = match access_level {
            AccessLevel::User => USER_LEVEL_COMMANDS
                .iter()
                .filter(|(command, _)| command.contains(line))
                .collect::<Vec<_>>(),
            AccessLevel::Admin => ADMIN_LEVEL_COMMANDS
                .iter()
                .filter(|(command, _)| command.contains(line))
                .collect::<Vec<_>>(),
        };

        if !res.is_empty() {
            return Command {
                kind: Kind::Command,
                line: res
                    .iter()
                    .map(|(command, _)| format!("{command}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            };
        }

        Command {
            kind: Kind::Error,
            line: String::from("No such command"),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.kind, self.line)
    }
}

impl From<(&AccessLevel, &Line)> for Command {
    fn from(command: (&AccessLevel, &Line)) -> Self {
        let (access_level, line) = command;

        match line {
            Line::Show(_property) => todo!("implement this please"),
            Line::Login => match access_level {
                AccessLevel::User => Self {
                    kind: Kind::Command,
                    line: String::from("logged in"),
                },
                AccessLevel::Admin => Self {
                    kind: Kind::Error,
                    line: String::from("command not found"),
                },
            },
            Line::Logout => match access_level {
                AccessLevel::User => Self {
                    kind: Kind::Error,
                    line: String::from("command not found"),
                },
                AccessLevel::Admin => Self {
                    kind: Kind::Command,
                    line: String::from("logged out"),
                },
            },
            Line::Update(version) => match access_level {
                AccessLevel::User => Self {
                    kind: Kind::Error,
                    line: String::from("command not found"),
                },
                AccessLevel::Admin => Self {
                    kind: Kind::Command,
                    line: format!("updated to version {version}"),
                },
            },
            Line::Save(in_file, out_file) => match access_level {
                AccessLevel::User => Self {
                    kind: Kind::Error,
                    line: String::from("command not found"),
                },
                AccessLevel::Admin => Self {
                    kind: Kind::Command,
                    line: format!("saved '{in_file}' to '{out_file}'"),
                },
            },
            Line::Clear => Self {
                kind: Kind::Clear,
                line: String::new(),
            },
            Line::Empty => Self {
                kind: Kind::Command,
                line: String::new(),
            },
            Line::Error(error) => Self {
                kind: Kind::Error,
                line: error.to_string(),
            },
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
struct TerminalCurrentLine(String);

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
enum AccessLevel {
    #[default]
    User,
    Admin,
}

#[derive(Component, Default, Deref, DerefMut)]
struct TerminalHistory(Vec<Command>);

#[derive(Component, Deref)]
struct TerminalHostname(String);

impl std::fmt::Display for TerminalHostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

#[derive(Component)]
struct TerminalBackground;

#[derive(Component)]
struct TerminalText;

#[derive(Component)]
struct CurrentText;

#[derive(Component)]
struct HistoryText;

#[derive(Component)]
struct TerminalItem;

fn startup(commands: Commands, asset_server: Res<AssetServer>) {
    instantiate_terminal(commands, asset_server, "test01".to_string());
}

fn instantiate_terminal(mut commands: Commands, asset_server: Res<AssetServer>, hostname: String) {
    let mut terminal_background = NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(70.)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::rgb(0.05, 0.05, 0.05).into(),
        ..default()
    };

    terminal_background.visibility = Visibility::Hidden;

    let mut current_text = TextBundle::from_sections([
        TextSection::new(
            format!("{hostname}> "),
            TextStyle {
                font: asset_server.load("fonts/fira-code/regular.ttf"),
                font_size: 16.0,
                color: Color::GREEN,
            },
        ),
        TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/fira-code/medium.ttf"),
            font_size: 16.0,
            color: Color::GREEN,
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
    });

    current_text.visibility = Visibility::Hidden;

    let mut history_text = TextBundle::from_sections([TextSection::from_style(TextStyle {
        font: asset_server.load("fonts/fira-code/medium.ttf"),
        font_size: 16.0,
        color: Color::GREEN,
    })])
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        ..default()
    });

    history_text.visibility = Visibility::Hidden;

    commands
        .spawn((terminal_background, TerminalBackground, TerminalItem))
        .with_children(|parent| {
            parent.spawn((current_text, CurrentText, TerminalText, TerminalItem));
            parent.spawn((history_text, HistoryText, TerminalText, TerminalItem));
        });

    commands.spawn((
        TerminalCurrentLine::default(),
        TerminalHistory::default(),
        TerminalHostname(hostname),
    ));
}

fn hide(mut terminal_item: Query<&mut Visibility, With<TerminalItem>>) {
    for mut visibility in &mut terminal_item {
        *visibility = Visibility::Hidden;
    }
}

fn show(mut terminal_item: Query<&mut Visibility, With<TerminalItem>>) {
    for mut visibility in &mut terminal_item {
        *visibility = Visibility::Inherited;
    }
}

fn _elevate_privileges(
    mut text: Query<&mut Text, With<CurrentText>>,
    hostname: Query<&TerminalHostname>,
) {
    let hostname = hostname.single();

    let text = &mut text.single_mut();

    text.sections[0].value = format!("{hostname}#");
}

fn _deelevate_privileges(
    mut text: Query<&mut Text, With<CurrentText>>,
    hostname: Query<&TerminalHostname>,
) {
    let hostname = hostname.single();

    let text = &mut text.single_mut();

    text.sections[0].value = format!("{hostname}>");
}

fn input(
    current_state: Res<State<AccessLevel>>,
    mut update_state: ResMut<NextState<AccessLevel>>,
    mut event_reader: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut terminal: Query<(&mut TerminalCurrentLine, &mut TerminalHistory)>,
    mut text: Query<&mut Text, With<CurrentText>>,
) {
    let (mut current, mut history) = terminal.single_mut();
    let text = &mut text.single_mut();
    let access_level = &current_state.0;

    text.sections[1].value = (*current).to_string();

    for event in event_reader.iter() {
        // Handle 'backspace'
        if event.char == char::from(0x08) {
            (*current).pop();
        } else if event.char == '?' {
            let line = (*current).trim();
            (*history).push(Command::query(access_level, line));
        } else {
            (*current).push(event.char);
        }
    }

    if keys.just_pressed(KeyCode::Return) {
        let line = Line::from((*current).trim());

        match access_level {
            AccessLevel::User => match line {
                Line::Login => update_state.set(AccessLevel::Admin),
                _ => ()
            },
            AccessLevel::Admin => match line {
                Line::Logout => update_state.set(AccessLevel::User),
                _ => ()
            },
        }

        let command = Command::from((access_level, &line));

        if let Kind::Clear = command.kind {
            (*history).clear();
        } else {
            (*history).push(command);
        }

        (*current).clear();
    }
}

fn text_update_system(
    mut text: Query<&mut Text, With<HistoryText>>,
    terminal: Query<&TerminalHistory>,
) {
    let history = terminal.single();

    for mut text in &mut text {
        text.sections[0].value = pretty_print(&history);
    }
}

fn pretty_print(commands: &Vec<Command>) -> String {
    commands
        .iter()
        .map(|command| command.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
