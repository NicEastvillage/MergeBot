mod player;
mod messages;

use crate::bot::Bot;
use crate::game::{Board, perft};
use std::time::Instant;

mod bot;
mod game;

fn main() {
    println!("Hello, world!");

    for x in 1..5 {
        let now = Instant::now();
        let p = perft(&mut Board::new(), x);
        println!("Perft {} = {} ({} ms)", x, p, (now.elapsed().as_micros() as f64) / 1000f64);
    }
}
