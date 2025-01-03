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
    assert_eq!(mcts.root.borrow().N, 10, "One run visits the root and all it's children. 1 + 9 = 10 = root.N");
    mcts.run();
    assert_eq!(mcts.root.borrow().N, 11, "one more run has only one path up to root so + 1 more");

}

