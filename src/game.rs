use std::hash::{Hash, Hasher};
use std::path::Display;
use std::fmt;
use serde::export::Formatter;
use std::num::ParseIntError;

const MAX_TIER: i32 = 1;

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
    pub turn: i32,
    pub grid: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        return Board {
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
        if moving_piece.team != self.turn { return Err(BoardError::NotCurrentPlayersPiece); }

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

        self.turn = 1 - self.turn;

        Ok(())
    }

    pub fn find_all_actions(&self) -> Vec<Action> {
        let mut moves: Vec<Action> = vec![];
        let mut only_capturing = false;

        for x in 0..8 {
            for y in 0..8 {
                let from = (x + y * 8) as usize;
                if let Some(piece) = &self.grid[from] {

                    if piece.team != self.turn { continue; }

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
        write!(f, "current player: {}", self.turn)
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.turn.hash(state);
        for p in self.grid.iter() {
            p.hash(state);
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        if self.turn != other.turn { return false; }
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
        let parts: Vec<&str> = action.split(' ').collect();
        Ok(Action { from: parts[0].parse()?, to: parts[1].parse()? })
    }

    pub fn to_string(self) -> String {
        format!("{} {}", self.from, self.to)
    }
}

pub enum BoardError {
    OutOfBounds,
    NotCurrentPlayersPiece,
    InvalidMove,
}