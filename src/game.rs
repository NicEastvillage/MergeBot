use std::hash::{Hash, Hasher};
use std::path::Display;
use std::fmt;
use serde::export::Formatter;
use std::num::ParseIntError;
use crate::game::Status::HitTurnLimit;

const MAX_TIER: i32 = 1;
const MAX_TURN: i32 = 30;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Piece {
    team: i32,
    tier: i32,
}

impl Piece {
    pub fn move_len(&self) -> i32 {
        return self.tier + 2
    }
}

fn piece(team: i32) -> Option<Piece> {
    return Some(Piece { team, tier: 0 })
}

#[derive(Clone)]
pub struct Board {
    pub current_player: i32,
    pub grid: [Option<Piece>; 64],
    pub turn: i32,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            current_player: 0,
            turn: 0,
            grid: [
                piece(0), piece(0), piece(0), piece(0), None,     None,     None,     None,
                piece(0), piece(0), piece(0), None,     None,     None,     None,     None,
                piece(0), piece(0), None,     None,     None,     None,     None,     None,
                piece(0), None,     None,     None,     None,     None,     None,     None,
                None,     None,     None,     None,     None,     None,     None, piece(1),
                None,     None,     None,     None,     None,     None, piece(1), piece(1),
                None,     None,     None,     None,     None, piece(1), piece(1), piece(1),
                None,     None,     None,     None, piece(1), piece(1), piece(1), piece(1),
            ],
        };
    }

    /// Does not check length and direction correctness of the action
    pub fn perform(&mut self, action: &Action) -> Result<(), BoardError> {
        if !Board::index_in_bounds(action.from) || !Board::index_in_bounds(action.to) {
            return Err(BoardError::OutOfBounds);
        }
        if action.from == action.to { return Err(BoardError::InvalidMove) }

        let moving_piece = self.grid[action.from].ok_or(BoardError::NotCurrentPlayersPiece)?;
        if moving_piece.team != self.current_player { return Err(BoardError::NotCurrentPlayersPiece); }

        let target = &self.grid[action.to];
        match target {
            None => {
                self.grid[action.from] = None;
                self.grid[action.to] = Some(moving_piece);
            },
            Some(target) => {
                if target.team == moving_piece.team && target.tier == moving_piece.tier {
                    // Merge
                    self.grid[action.from] = None;
                    self.grid[action.to] = Some(Piece { team: moving_piece.team, tier: moving_piece.tier + 1 });

                } else if target.team != moving_piece.team && target.tier <= moving_piece.tier {
                    // Capture
                    self.grid[action.from] = None;
                    self.grid[action.to] = Some(moving_piece);
                }
            }
        }

        self.current_player = 1 - self.current_player;
        self.turn += 1;

        Ok(())
    }

    pub fn status(&self) -> Status {
        if self.turn >= MAX_TURN {
            return Status::HitTurnLimit;
        }

        for team in 0..2 {
            let mut count = 0;
            for i in 0..64 {
                if let Some(piece) = self.grid[i] {
                    if piece.team == team {
                        count += 1;
                    }
                }
            }
            if count <= 1 {
                return Status::Won { winner: 1 - team }
            }
        }

        return Status::Running;
    }

    pub fn find_all_actions(&self) -> Vec<Action> {
        let mut moves: Vec<Action> = vec![];
        let mut only_capturing = false;

        for x in 0..8 {
            for y in 0..8 {
                let from = (x + y * 8) as usize;
                if let Some(piece) = &self.grid[from] {

                    if piece.team != self.current_player { continue; }

                    let move_len = piece.move_len();

                    for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter() {
                        for m in 1..move_len + 1 {
                            let tx = x + dx * m;
                            let ty = y + dy * m;
                            if Board::pos_in_bounds(tx, ty) {
                                let to = (tx + ty * 8) as usize;
                                let dest = &self.grid[to];
                                if only_capturing {
                                    // At least one capturing is possible, so we only consider capturing now
                                    if let Some(enemy) = dest {
                                        if enemy.team != piece.team && enemy.tier <= piece.tier {
                                            moves.push(Action { from, to });
                                            break;
                                        }
                                    }
                                } else {
                                    match dest {
                                        None => {
                                            moves.push(Action { from, to });
                                        },
                                        Some(target) => {
                                            if target.team == piece.team && target.tier == piece.tier && piece.tier < MAX_TIER {
                                                // Merge
                                                moves.push(Action { from, to });
                                            } else if target.team != piece.team && target.tier <= piece.tier {
                                                // Capture
                                                moves.clear();
                                                only_capturing = true;
                                                moves.push(Action { from, to });
                                                break;
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return moves;
    }

    pub fn human_readable(&self) -> String {
        let mut res = "  a b c d e f g h\n".to_string();
        for y in 0..8 {
            res.push_str(&format!("{} ", y + 1));
            for x in 0..8 {
                let c = match self.grid[x + y * 8] {
                    None => '.',
                    Some(p) => {
                        if p.team == 0 && p.tier == 0 { 'w' }
                        else if p.team == 0 && p.tier == 1 { 'W' }
                        else if p.team == 1 && p.tier == 0 { 'r' }
                        else if p.team == 1 && p.tier == 1 { 'R' }
                        else { '?' }
                    },
                };
                res.push_str(&format!("{} ", c));
            }
            res.push_str(&format!("\n"));
        }
        res.push_str(&format!("Turn {}, {}", self.turn, &["white", "red"][self.current_player as usize]));
        return res;
    }

    pub fn index_in_bounds(pos: usize) -> bool {
        return pos < 64;
    }

    pub fn pos_in_bounds(x: i32, y: i32) -> bool {
        return 0 <= x && x < 8 && 0 <= y && y < 8;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                let c = match self.grid[x + y * 8] {
                    None => '.',
                    Some(p) => {
                        if p.team == 0 && p.tier == 0 { 'w' }
                        else if p.team == 0 && p.tier == 1 { 'W' }
                        else if p.team == 1 && p.tier == 0 { 'r' }
                        else if p.team == 1 && p.tier == 1 { 'R' }
                        else { '?' }
                    },
                };
                write!(f, "{} ", c);
            }
            write!(f, "\n");
        }
        write!(f, "Turn {}, {}", self.turn, &["white", "red"][self.current_player as usize])
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_player.hash(state);
        for p in self.grid.iter() {
            p.hash(state);
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        if self.current_player != other.current_player { return false; }
        for i in 0..64 {
            if self.grid[i] != other.grid[i] {
                return false;
            }
        }
        return true;
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Action {
    pub from: usize,
    pub to: usize,
}

impl Action {
    pub fn from(action: &str) -> Result<Action, ParseIntError> {
        let parts: Vec<&str> = action.trim().split(' ').collect();
        Ok(Action { from: parts[0].parse()?, to: parts[1].parse()? })
    }

    pub fn to_string(self) -> String {
        format!("{} {}", self.from, self.to)
    }

    pub fn human_readable(&self) -> String {
        let fy = self.from / 8;
        let fx = self.from - fy * 8;
        let ty = self.to / 8;
        let tx = self.to - ty * 8;
        let labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        return format!("{}{} {}{}", labels[fx], fy + 1, labels[tx], ty + 1);
    }
}

pub enum BoardError {
    OutOfBounds,
    NotCurrentPlayersPiece,
    InvalidMove,
}

pub enum Status {
    Running,
    Won { winner: i32 },
    HitTurnLimit,
}