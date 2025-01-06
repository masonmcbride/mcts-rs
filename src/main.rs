use std::rc::Rc;
use ndarray::Array2;
use mcts_rs::mcts::MCTS;
use mcts_rs::game::Game;
use mcts_rs::games::tictactoe::TicTacToe;

fn main() {

    // 1. Create an empty 3x3 board.
    let board = Array2::<i8>::zeros((3, 3));
    let other_board = { let mut b = board.clone(); b[[1,1]] = 1; b};

    // 2. Create a TicTacToe instance and get the TicTacToeState from the board.
    let mut tic_tac_toe = TicTacToe::new();
    let state = tic_tac_toe.get_state(&board);
    let other_state = tic_tac_toe.get_state(&other_board);

    // 3. Create an MCTS using that state.
    let mut mcts = MCTS::new(tic_tac_toe, state.clone());

    // 4. Call get_node on the same state.
    let node = mcts.get_node(state.clone());

    // 5. Print out the board from the node’s game_state.
    println!("Board from MCTSNode:\n{}", node.borrow().game_state);
    println!("Strong count: {}", Rc::strong_count(&state));
    println!("Weak count: {}", Rc::weak_count(&state));
    println!("Number of nodes in Tree: {}", mcts.nodes.len());
    println!("adding a new state to the tree.");
    mcts.get_node(other_state.clone());
    println!("Number of nodes in Tree now: {}", mcts.nodes.len());
}