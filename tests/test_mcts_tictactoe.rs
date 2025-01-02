use ndarray::prelude::*;
use mcts_rs::games::tictactoe::{TicTacToeState, TicTacToe};
use mcts_rs::mcts::MCTS;

#[test]
fn test_one_run_expands_and_selects_one() {

    let mut tictactoe = TicTacToe::new();
    let empty_board = Array2::zeros((3, 3)); // Initial empty board key
    let new_game = tictactoe.get_state(&empty_board);
    let mut mcts = MCTS::new(new_game.clone(), tictactoe);
    mcts.run();

    //assert_eq!(mcts.root.borrow().N, 10, "The number of states should be 5478");
}

