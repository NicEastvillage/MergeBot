mod messages;

use crate::bot::Bot;
use crate::game::Board;

mod bot;
mod game;

fn main() {
    println!("Hello, world!");
    println!("{}", Board::new())
}
