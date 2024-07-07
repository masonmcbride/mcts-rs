use std::rc::Rc;
use ndarray::prelude::*;
mod games;
mod mcts;
use games::tictactoe::{TicTacToe, TicTacToeState};

fn main() {
    let mut tictactoe = TicTacToe::new();
    let empty_board = Array2::zeros((3, 3)); // Initial empty board key
    let initial_state = tictactoe.get_state(&empty_board);
    let mut mcts = MCTS::new(initial_state);
    mcts.search(1000);
    
    if let Some(best_child) = mcts.root.children.iter()
                                                .max_by(|a, b| a.borrow().Q()
                                                .partial_cmp(&b.borrow().Q())
                                                .unwrap()) {
        let best_state = &best_child.borrow().state.board;
        println!("Winning move board state:\n{:?}", best_state);
    } else {
        println!("No children found");
    }

}
