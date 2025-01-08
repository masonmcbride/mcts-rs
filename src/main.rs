use ndarray::Array2;
use mcts_rs::mcts::MCTS;
use mcts_rs::game::Game;
use mcts_rs::games::tictactoe::TicTacToe;

fn main() {

    let mut tic_tac_toe = TicTacToe::new();

    let board = Array2::<i8>::zeros((3,3));
    let state = tic_tac_toe.get_state(&board);

    let mut mcts = MCTS::new(tic_tac_toe, state.clone());
    mcts.search(1000000);
    println!("done searching");
}
