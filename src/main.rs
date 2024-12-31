use mcts_rs::mcts::MCTS;
use mcts_rs::games::tictactoe::TicTacToe;
use ndarray::Array2;

fn main() {

    // 1. Create an empty 3x3 board.
    let board = Array2::<i8>::zeros((3, 3));

    // 2. Create a TicTacToe instance and get the TicTacToeState from the board.
    let mut tic_tac_toe = TicTacToe::new();
    let state = tic_tac_toe.get_state(&board);

    // 3. Create an MCTS using that state.
    let mut mcts = MCTS::new(state.clone());

    // 4. Call get_node on the same state.
    let node = mcts.get_node(state.clone());

    // 5. Print out the board from the node’s game_state.
    println!("Board from MCTSNode:\n{}", node.game_state);
}