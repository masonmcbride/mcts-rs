use std::rc::Rc;
extern crate mcts_rs;
use ndarray::prelude::*;
use mcts_rs::games::tictactoe::{TicTacToeState, TicTacToe};

#[test]
fn test_tictactoe_finds_all_states() {
    fn explore_states(game: &mut TicTacToe, state: Rc<TicTacToeState>) {
        for action in state.all_legal_actions.iter() {
            let next_state = game.transition(Rc::clone(&state), *action);
            explore_states(game, next_state);
        }
    }
    let mut tictactoe = TicTacToe::new();
    let empty_board = Array2::zeros((3, 3)); // Initial empty board key
    let initial_state = tictactoe.get_state(&empty_board);
    explore_states(&mut tictactoe, initial_state);

    for (board, state) in tictactoe.game_states.iter() {
        assert_eq!(Rc::strong_count(state), 1 , "Strong count for state {:?} is not equal to 1", board);
    }

    // Assert that the number of states is exactly 5478
    assert_eq!(tictactoe.game_states.len(), 5478, "The number of states should be 5478");
}

