use std::ops::{Index, IndexMut};

use bevy::prelude::*;

use super::{PreviousIterator, SHELL_HEIGHT, SHELL_WIDTH};

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

#[derive(Bundle, Default)]
pub struct ShellModel {
    pub hostname: Hostname,
    pub shell_data: ShellData,
    pub current_line: CurrentPosition,
    pub access_level: AccessLevel,
}

#[derive(Component, Default, Deref, DerefMut, Clone)]
pub struct Hostname(pub String);

#[derive(Component, Default)]
pub struct Selected;

#[derive(Component, Deref, DerefMut)]
pub struct ShellData(pub [[char; SHELL_WIDTH]; SHELL_HEIGHT]);

impl ShellData {
    fn push(&mut self, current_position: &mut CurrentPosition, character: char) {
        self[*current_position] = character;

        current_position.next();
    }

    fn push_str(&mut self, current_position: &mut CurrentPosition, string: &str) {
        for character in string.chars() {
            self.push(current_position, character);
        }
    }

    pub fn pop(&mut self, current_position: &mut CurrentPosition) -> char {
        current_position.previous();

        let removed_character = self[*current_position];
        self[*current_position] = '\0';

        removed_character
    }

    pub fn clear(&mut self, current_position: &mut CurrentPosition) {
        self.0 = [['\0'; SHELL_WIDTH]; SHELL_HEIGHT];
        current_position.first();
    }

    pub fn insert(
        event_reader: &mut EventReader<ReceivedCharacter>,
        mut shell_data: Mut<ShellData>,
        mut current_position: Mut<CurrentPosition>,
        access_level: Mut<AccessLevel>,
    ) {
        for event in event_reader.iter() {
            if event.char == '?' {
                let line = shell_data.0[(*current_position).0]
                    .iter()
                    .collect::<String>();
                shell_data.push_str(
                    &mut current_position,
                    &ShellCommands::describe(*access_level, &line),
                );
            } else {
                shell_data.push(&mut current_position, event.char);
            }
        }
    }
}

impl Index<CurrentPosition> for ShellData {
    type Output = char;

    fn index(&self, index: CurrentPosition) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<CurrentPosition> for ShellData {
    fn index_mut(&mut self, index: CurrentPosition) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl Default for ShellData {
    fn default() -> Self {
        Self([['\0'; SHELL_WIDTH]; SHELL_HEIGHT])
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct CurrentPosition(usize, usize);

impl CurrentPosition {
    fn new_line(&mut self) {
        self.0 += 1;
        self.1 = 0;
        self.0 %= SHELL_HEIGHT;
    }
}

impl Iterator for CurrentPosition {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;
        self.0 += self.1 / SHELL_WIDTH;

        if self.0 != SHELL_HEIGHT - 1 {
            self.1 = self.1 % SHELL_WIDTH;
        } else if self.1 == SHELL_WIDTH {
            return None;
        }

        Some(CurrentPosition(self.0, self.1))
    }
}

impl PreviousIterator for CurrentPosition {
    fn previous(&mut self) -> Option<Self::Item> {
        if self.1 == 0 {
            if self.0 != 0 {
                self.0 -= 1;
            }

            self.1 = SHELL_WIDTH - 1;
        } else {
            self.1 -= 1;
        }

        if self.1 == 0 && self.0 == 0 {
            return None;
        }

        Some(CurrentPosition(self.0, self.1))
    }
}

pub enum ShellProperty {}

impl TryFrom<&str> for ShellProperty {
    type Error = String;

    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        Err("unknown property".to_string())
    }
}

pub enum ShellCommands {
    Show(ShellProperty),
    Login,
    Logout,
    Clear,
    Update(String),
    Save(String, String),
    Empty,
    Error(String),
}

impl ShellCommands {
    pub fn describe(access_level: AccessLevel, query: &str) -> String {
        match access_level {
            AccessLevel::User => format!(
                "These are the available commands: \n- {}",
                USER_LEVEL_COMMANDS
                    .iter()
                    .filter(|(command, _)| command.contains(query))
                    .map(|(command, description)| format!("{command} {description}"))
                    .collect::<Vec<_>>()
                    .join("\n- ")
            ),

            AccessLevel::Admin => format!(
                "These are the available commands: \n- {}",
                ADMIN_LEVEL_COMMANDS
                    .iter()
                    .filter(|(command, _)| command.contains(query))
                    .map(|(command, description)| format!("{command} {description}"))
                    .collect::<Vec<_>>()
                    .join("\n- ")
            ),
        }
    }

    pub fn process(
        access_level: &mut Mut<AccessLevel>,
        shell_data: &mut Mut<ShellData>,
        current_position: &mut Mut<CurrentPosition>,
        hostname: Hostname,
    ) {
        let prompt = match **access_level {
            AccessLevel::User => format!("{}> ", *hostname),
            AccessLevel::Admin => format!("{}# ", *hostname),
        };

        let prompt_length = prompt.len();

        let line = shell_data.0[(**current_position).0]
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx >= &prompt_length)
            .map(|(_, character)| character)
            .collect::<String>();

        let line = ShellCommands::from(line);

        let result = match line {
            ShellCommands::Show(_property) => format!("implement this please"),
            ShellCommands::Login => {
                if let AccessLevel::User = **access_level {
                    **access_level = AccessLevel::Admin;
                }

                format!("logged in")
            }
            ShellCommands::Logout => {
                if let AccessLevel::Admin = **access_level {
                    **access_level = AccessLevel::User;
                    format!("logged out")
                } else {
                    format!("command not found")
                }
            }
            ShellCommands::Clear => {
                shell_data.clear(current_position);
                format!("")
            }
            ShellCommands::Update(version) => format!("updated to version {version}"),
            ShellCommands::Save(in_file, out_file) => {
                if let AccessLevel::Admin = **access_level {
                    format!("saved '{in_file}' to '{out_file}'")
                } else {
                    format!("command not found")
                }
            }
            ShellCommands::Empty => format!(""),
            ShellCommands::Error(error) => format!("error: {error}"),
        };

        current_position.new_line();
        shell_data.push_str(current_position, &result);

        current_position.new_line();
        shell_data.push_str(current_position, &format!("{}", prompt));
    }
}

impl From<String> for ShellCommands {
    fn from(value: String) -> Self {
        Self::from(value.as_ref())
    }
}

impl From<&str> for ShellCommands {
    fn from(command: &str) -> Self {
        let command = command.split(" ").map(|s| s.trim()).collect::<Vec<_>>();

        let first_command = match command.get(0) {
            Some(&command) => command,
            None => return Self::Empty,
        };

        match first_command {
            "clear" => Self::Clear,
            "login" => Self::Login,
            "logout" => Self::Logout,
            "show" => {
                let property = match command.get(1) {
                    Some(&property) => property,
                    None => return Self::Error(String::from("not enough arguments, expected 1")),
                };

                match ShellProperty::try_from(property) {
                    Ok(property) => Self::Show(property),
                    Err(error) => Self::Error(error),
                }
            }
            "update" => {
                let version = match command.get(1) {
                    Some(&property) => property,
                    None => return Self::Error(String::from("not enough arguments, expected 1")),
                };

                Self::Update(version.to_string())
            }
            "save" => {
                let in_file = match command.get(1) {
                    Some(&file) => file,
                    None => return Self::Error(String::from("not enough arguments, expected 2")),
                };

                let out_file = match command.get(1) {
                    Some(&file) => file,
                    None => return Self::Error(String::from("not enough arguments, expected 2")),
                };

                Self::Save(in_file.to_string(), out_file.to_string())
            }
            "" => Self::Empty,
            _ => Self::Error(format!("unknown command '{first_command}'")),
        }
    }
}

#[derive(Component, Default, Clone, Copy)]
pub enum AccessLevel {
    #[default]
    User,
    Admin,
}
