use std::collections::HashMap;
use super::games::tictactoe::{TicTacToe, TicTacToeState};


pub struct MCTSNode {
    game_state: i32,
    is_terminal: i32,
    is_expanded: bool,
    children: Vec<i32>,
    N: u32,
    W: u32,
    P: f64,
    results: HashMap<i8, u32>
}

impl MCTSNode {
    pub fn new(game_state: TicTacToeState) -> MCTSNode {
        let is_terminal = -32;
        MCTSNode {
            game_state,
            is_terminal,
            is_expanded: false,
            children: Vec::new(),
            N: 0,
            W: 0,
            P: 1.0,
            results: vec![(1,0),(-1,0),(0,0)].into_iter().collect()
        }
    }

    pub fn Q(&self) -> f64 {
        self.W as f64 / self.N as f64
    }
}