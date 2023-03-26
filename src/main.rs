use bevy::prelude::*;
use game::Game;

mod game;
mod object;
mod physics;
mod terminal;

fn main() {
    App::new().add_plugin(Game).run();
}
