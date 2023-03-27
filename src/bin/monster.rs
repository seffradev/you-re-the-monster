use bevy::prelude::*;
use you_re_the_monster::game::Game;

fn main() {
    App::new().add_plugin(Game).run();
}
