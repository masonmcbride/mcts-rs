use std::rc::Rc;
use std::cell::RefCell;
use ndarray::prelude::*;
mod games;
mod mcts;
use games::tictactoe::{TicTacToe, TicTacToeState};


fn explore_states(game: &mut TicTacToe, state: Rc<RefCell<TicTacToeState>>) {
    for action in state.borrow().all_legal_actions.iter() {
        let next_state = game.transition(Rc::clone(&state), *action);
        explore_states(game, next_state);
    }
}

fn main() {
    let mut tictactoe = TicTacToe::new();
    let empty_board = Array2::zeros((3, 3)); // Initial empty board key
    let initial_state = tictactoe.get_state(&empty_board);
    explore_states(&mut tictactoe, initial_state);
    for (board, state) in tictactoe.game_states.iter() {
        println!("Board: {:?}, Strong count: {}", board, Rc::strong_count(state));
    }

    println!("Total number of states: {}", tictactoe.game_states.len());
}
