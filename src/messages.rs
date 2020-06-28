use serde::{Deserialize};
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Result};

#[derive(Debug, Deserialize)]
#[serde(tag = "message_type")]
pub enum Message {
    #[serde(rename = "start")]
    Start(StartMsg),
    #[serde(rename = "move")]
    Move(MoveMsg),
    #[serde(rename = "error")]
    Error(ErrorMsg),
    #[serde(rename = "end")]
    End(EndMsg),
}

#[derive(Debug, Deserialize)]
pub struct StartMsg {
    pub message_type: String,
    pub message: String,
    pub your_side: i64,
    pub your_turn: bool,
    pub full_board: String,
}

#[derive(Debug, Deserialize)]
pub struct MoveMsg {
    pub message_type: String,
    pub message: String,
    pub move_played: String,
    pub your_turn: bool,
    pub full_board: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorMsg {
    pub message_type: String,
    pub message: String,
    pub command_received: String,
}

#[derive(Debug, Deserialize)]
pub struct EndMsg {
    pub message_type: String,
    pub message: String,
    pub move_played: String,
    pub winner: i64,
    pub full_board: String,
}

fn receive(stream: &TcpStream) -> Result<Message> {
    let mut msg = String::new();
    BufReader::new(stream).read_line(&mut msg)?;
    // println!("{}", msg.as_str());
    Ok(serde_json::from_str::<Message>(msg.as_str())?)
}