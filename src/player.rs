use crate::game::{Action, Board, Status};

use std::{
    io::{prelude::*, BufReader, BufWriter, Result, Error, ErrorKind},
    net::TcpStream,
};
use crate::messages::Message;
use std::borrow::BorrowMut;

pub trait MergePlayer {
    fn get_action(&mut self) -> Action;
    fn update(&mut self, action: &Action);
}

pub fn play_online<F>(address: &str, factory: F) -> Result<()>
where
    F: FnOnce(i32) -> Box<dyn MergePlayer>
{
    // Create connection
    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    println!("Connected to {}", address);
    send(&mut writer, "ready\n");

    if let Message::Start(start_msg) = receive(&mut reader)? {
        println!("A game has started. Playing as {}", &["white", "red"][start_msg.your_side as usize]);
        let mut player = factory(start_msg.your_side);

        loop {
            match receive(&mut reader)? {
                Message::Start(_) => return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Received a second start message",
                )),
                Message::Error(err_msg) => println!("Error ({}): {}", err_msg.message, err_msg.message_type),
                Message::End(end_msg) => return Ok(()),
                Message::Move(move_msg) => {
                    if move_msg.your_turn {
                        player.update(&Action::from(&move_msg.move_played).unwrap());
                        let action = player.get_action();
                        send(&mut writer, &action.to_string());
                        player.update(&action);
                    }
                }
            };
        }

    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Did not receive a start message",
        ))
    }
}

fn receive<R: BufRead>(reader: &mut R) -> Result<Message> {
    let mut msg = String::new();
    reader.read_line(&mut msg)?;
    Ok(serde_json::from_str::<Message>(&msg)?)
}

fn send<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?; // Note: Messages are not guaranteed to end with newline
    writer.flush()
}

pub fn play_duel<P1, P2>(mut player_one: P1, mut player_two: P2, show_moves: bool)
where
    P1: MergePlayer,
    P2: MergePlayer,
{
    let mut board = Board::new();
    loop {
        let action = if board.current_player == 0 {
            player_one.get_action()
        } else {
            player_two.get_action()
        };

        board.perform(&action);
        player_one.update(&action);
        player_two.update(&action);

        if show_moves {
            println!("\n{} did: {}", &["white", "red"][1 - board.current_player as usize], &action.human_readable());
            println!("{}", board.human_readable());
        }

        let status = board.status();
        match status {
            Status::Won { winner } => {
                println!("\n{} won!", &["White", "Red"][winner as usize]);
                return;
            },
            Status::HitTurnLimit => {
                println!("\nGame over due to turn limit!");
                return;
            },
            _ => {}
        };
    }
}