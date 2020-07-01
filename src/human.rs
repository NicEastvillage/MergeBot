use crate::game::{Board, Action};
use crate::player::MergePlayer;

pub struct Human {
    board: Board,
    last_enemy_action: Option<Action>,
}

impl Human {
    pub fn new() -> Human {
        return Human {
            board: Board::new(),
            last_enemy_action: None,
        }
    }
}

impl MergePlayer for Human {
    fn get_action(&mut self) -> Action {
        if let Some(action) = self.last_enemy_action {
            println!("Enemy did: {}", action.human_readable());
        }
        println!("{}\nYour move:", self.board.human_readable());

        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        return Action::from(&line).expect("Invalid action, human error");
    }

    fn update(&mut self, action: &Action) {
        self.board.perform(action);
        self.last_enemy_action = Some(action.clone()) // Also own actions, well whatever
    }
}