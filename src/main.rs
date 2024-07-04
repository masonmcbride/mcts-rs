use ndarray::prelude::*;
mod games;
fn main() {
    let one_move_to_win: Array2<i8> = array![
        [1,-1,0],
        [1,1,-1],
        [-1,0,0]];
    //almost_won = TicTacToe.get_state(state=one_move_to_win)
    //mcts = MCTS(game_state=almost_won)
    //mcts.search(50) 
    //winning_move = max(mcts.root.children, key=lambda child: child.Q)
    println!("{:?}",one_move_to_win)
}
