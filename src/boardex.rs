use crate::game::{Board, Piece, Action};
use std::hint::unreachable_unchecked;

pub struct BoardExplorer {
    board: Board,
    undo_stack: Vec<UndoAction>,
}

impl BoardExplorer {
    pub fn new(board: Board) -> BoardExplorer {
        BoardExplorer {
            board,
            undo_stack: vec![],
        }
    }

    pub fn try_perform(&mut self, action: &Action) {
        let undo = UndoAction {
            from: action.from,
            from_piece: self.board.grid[action.from],
            to: action.to,
            to_piece: self.board.grid[action.to],
        };
        self.undo_stack.push(undo);
        self.board.perform(action);
    }

    pub fn undo(&mut self) {
        let undo = self.undo_stack.pop().expect("Undo stack is empty.");
        self.board.grid[undo.from] = undo.from_piece;
        self.board.grid[undo.to] = undo.to_piece;
        self.board.turn = 1 - self.board.turn;
    }
}

struct UndoAction {
    pub from: usize,
    pub from_piece: Option<Piece>,
    pub to: usize,
    pub to_piece: Option<Piece>,
}

// Perft 1 = 99 (0.086 ms)
// Perft 2 = 9409 (5.506 ms)
// Perft 3 = 791469 (715.917 ms)
// Perft 4 = 65152608 (31062.843 ms)
// Perft 5 = 4843034918 (2739432.218 ms)
pub fn perft(explorer: &mut BoardExplorer, depth: i32) -> u64 {
    if depth == 1 {
        return explorer.board.find_all_actions().len() as u64;

    } else {
        let mut total = 0;
        let actions = explorer.board.find_all_actions();

        for action in &actions {
            let b = explorer.board.clone();
            explorer.try_perform(action);
            total += perft(explorer, depth - 1);
            explorer.undo();
        }

        return total;
    }
}