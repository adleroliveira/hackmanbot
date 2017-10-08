extern crate hackmanbot;

use hackmanbot::game::Game;
use std::process;
use std::io;
use std::io::Write;

fn main() {
    let mut game = Game::new(io::stdin());

    game.start().unwrap_or_else(|err| {
        writeln!(&mut io::stderr(), "{}", err).expect("failed printing to stderr");
        process::exit(1);
    });
}
