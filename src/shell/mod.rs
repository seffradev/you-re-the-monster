use self::{
    controller::input,
    model::{Hostname, Selected, ShellModel},
    view::{hide, show, show_shell, ShellScreen, ShellView},
};
use crate::game::model::GameState;
use bevy::prelude::*;

pub mod controller;
pub mod model;
pub mod view;

const SHELL_WIDTH: usize = 80;
const SHELL_HEIGHT: usize = 24;

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_system(input.in_set(OnUpdate(GameState::Shell)))
            .add_systems((
                hide.in_schedule(OnExit(GameState::Shell)),
                show.in_schedule(OnEnter(GameState::Shell)),
                show_shell.in_set(OnUpdate(GameState::Shell)),
            ));
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let id = instantiate_shell(&mut commands, &asset_server, "test01");

    commands.entity(id).insert(Selected);

    instantiate_shell(&mut commands, &asset_server, "test02");
}

fn instantiate_shell(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    hostname: &str,
) -> Entity {
    let node_bundle = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::SpaceBetween,
            align_self: AlignSelf::Center,
            ..default()
        },
        background_color: Color::rgb(0.05, 0.05, 0.05).into(),
        visibility: Visibility::Hidden,
        ..default()
    };

    let shell_text = TextBundle::from_sections((0..SHELL_HEIGHT).map(|_| {
        TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/fira-code/regular.ttf"),
            font_size: 16.0,
            color: Color::GREEN,
        })
    }));

    commands
        .spawn((
            ShellModel {
                hostname: Hostname(String::from(hostname)),
                ..default()
            },
            ShellView {
                node_bundle,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((shell_text, ShellScreen));
        })
        .id()
}

trait PreviousIterator: Iterator {
    fn previous(&mut self) -> Option<Self::Item>;

    #[inline]
    fn first(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }

        PreviousIterator::fold(self, None, some)
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.previous() {
            accum = f(accum, x);
        }
        accum
    }
}
