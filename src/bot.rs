
use crate::game::{Board, Action, Status, Piece};
use crate::player::MergePlayer;
use rand::Rng;
use crate::boardex::BoardExplorer;
use std::cmp::max;
use std::fs::read_to_string;

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
        let color = 1 - self.board.current_player * 2;

        let mut best_value = -std::i32::MAX;

        let mut actions = self.board.find_all_actions();
        let mut actions: Vec<(Action, i32)> = actions
            .drain(..)
            .map(|act| {
                let value = negamax(&mut BoardExplorer::new(self.board.clone()), 3, -std::i32::MAX, std::i32::MAX, color);
                best_value = max(best_value, value);
                (act, value)
            }

            )
            .collect();

        let best: Vec<Action> = actions.drain(..)
            .filter_map(|(act, val)| {
                if val == best_value {
                    Some(act)
                } else {
                    None
                }
            }).collect();

        let mut rng = rand::thread_rng();
        return best[rng.gen_range(0, best.len())];
    }

    fn update(&mut self, action: &Action) {
        self.board.perform(action);
    }
}

fn negamax(bex: &mut BoardExplorer, depth: i32, alpha: i32, beta: i32, color: i32) -> i32 {
    let game_over = match bex.board.status() {
        Status::Running => false,
        Status::Won { .. } | Status::HitTurnLimit => true,
    };
    if depth == 0 || game_over {
        return color * heuristic(&bex.board);
    }

    let mut alpha = alpha;
    let mut value = std::i32::MIN;
    let actions = bex.board.find_all_actions();

    let depth = if actions.len() <= 3 {
        // A capture is likely happening, so we increase depth for this branch
        depth + 1
    } else {
        depth
    };

    for action in actions {
        bex.try_perform(&action);
        value = max(value, -negamax(bex, depth - 1, -beta, -alpha, -color));
        bex.undo();

        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    return value;
}

fn heuristic(board: &Board) -> i32 {
    return match board.status() {
        Status::Running => {
            let mut value = 0;
            for piece in board.grid.iter() {
                if let Some(Piece { team, tier }) = piece {
                    let color = 1 - *team * 2;
                    value += (3 + 3 * tier) * color;
                }
            }
            value
        },
        Status::Won { winner } => {
            if winner == 0 {
                999999
            } else {
                -999999
            }
        },
        Status::HitTurnLimit => 0,
    };
}




