use crate::game::Action;

use std::{
    io::{prelude::*, BufReader, BufWriter, Result, Error, ErrorKind},
    net::TcpStream,
};
use crate::messages::Message;
use std::borrow::BorrowMut;

pub trait MergePlayer {
    fn get_move(&mut self) -> Action;
    fn incoming_move(&mut self, action: Action);
}

pub fn play<F>(address: &str, factory: F) -> Result<()>
where
    F: FnOnce(i32) -> Box<dyn MergePlayer>
{
    // Create connection
    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    println!("Connected to {}", address);

    if let Message::Start(start_msg) = receive(&mut reader)? {
        // A game has started
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
                        player.incoming_move(Action::from(&move_msg.move_played).unwrap());
                        let action = player.get_move();
                        send(&mut writer, &action.to_string());
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
    writer.write_all(msg.as_bytes()) // Note: Messages are not guaranteed to end with newline
}