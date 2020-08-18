mod game2048terminal;
mod game2048core;

use game2048terminal::Game2048Terminal;
use std::io::{stdout};

fn main() {
    let mut out = stdout();
    let mut game = Game2048Terminal::new(&mut out);

    match game.run() {
        _ => {}
    }
}

