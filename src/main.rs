use ndarray::prelude::*;
mod games;

use games::tictactoe::TicTacToe;
fn main() {
    let one_move_to_win: Array2<i8> = array![
        [1,-1,0],
        [1,1,-1],
        [-1,0,0]];
    let mut tictactoe = TicTacToe::new();
    let almost_won = tictactoe.get_state(one_move_to_win);
    //let mcts = MCTS::new(almost_won)
    //mcts.search(50) 
    //winning_move = max(mcts.root.children, key=lambda child: child.Q)
    println!("{:?}", &almost_won)
}
