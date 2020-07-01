mod human;
mod boardex;
mod player;
mod messages;

use crate::bot::Bot;
use crate::game::{Board, Action};
use crate::human::Human;
use std::time::Instant;
use crate::boardex::{perft, BoardExplorer};
use crate::player::{play_online, play_duel};

mod bot;
mod game;

fn main() {
    println!("Hello, world!");

    play_duel(Bot::new(Board::new()), Bot::new(Board::new()), true);
}
