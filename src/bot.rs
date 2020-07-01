
use crate::game::{Board, Action};
use crate::player::MergePlayer;
use rand::Rng;

pub struct Bot {
    board: Board,
}

impl Bot {
    pub fn new(board: Board) -> Bot {
        return Bot {
            board
        }
    }
}

impl MergePlayer for Bot {
    fn get_action(&mut self) -> Action {
        let actions = self.board.find_all_actions();
        let mut rng = rand::thread_rng();
        return actions[rng.gen_range(0, actions.len())];
    }

    fn update(&mut self, action: &Action) {
        self.board.perform(action);
    }
}