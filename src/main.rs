extern crate hackmanbot;

use hackmanbot::game::Game;
use std::process;

fn main() {
    let mut game = Game::new(std::io::stdin());

    game.start().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
