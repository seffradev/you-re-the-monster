use bevy::prelude::*;
use game::Game;

mod game;
mod player;
mod terminal;

fn main() {
    App::new().add_plugin(Game).run();
}
