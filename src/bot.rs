
use crate::game::{Board, Action};

pub struct Bot {
    board: Board,
}

impl Bot {
    pub fn new(board: Board) -> Bot {
        return Bot {
            board
        }
    }

    pub fn next_move(self) -> Action {
        Action { from: 0, to: 0 }
    }
}