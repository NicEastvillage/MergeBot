mod boardex;
mod player;
mod messages;

use crate::bot::Bot;
use crate::game::Board;
use std::time::Instant;
use crate::boardex::{perft, BoardExplorer};

mod bot;
mod game;

fn main() {
    println!("Hello, world!");

    for x in 1..6 {
        let now = Instant::now();
        let p = perft(&mut BoardExplorer::new(Board::new()), x);
        println!("Perft {} = {} ({} ms)", x, p, (now.elapsed().as_micros() as f64) / 1000f64);
    }
}
