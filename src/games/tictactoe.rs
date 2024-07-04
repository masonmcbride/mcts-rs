use ndarray::prelude::*;
use std::collections::HashMap;
struct TicTacToeState {
    state: Array2<i8>,
    player: i8,
    result: Option<i8>,
    is_terminal: bool,
    all_legal_actions: Array1<(usize,usize)>
}

impl TicTacToeState {
    pub fn new(state: Array2<i8>) -> Self {
        let player: i8 = if state.sum() <= 0 { 1 } else { -1 };
        let result: Option<i8> = Self::game_result(&state);
        let is_terminal: bool = result.is_some();
        let all_legal_actions:Array1<(usize, usize)> = if !is_terminal {
            state.indexed_iter()
                .filter(|&((_,_),&value)| value == 0)
                .map(|((i,j),_)| (i,j))
                .collect::<Array1<(usize,usize)>>()
        } else {
            Array1::from_shape_vec(0, Vec::new()).unwrap()
        };
        TicTacToeState {
            state,
            player,
            result,
            is_terminal,
            all_legal_actions
        }

    }

    fn game_result(state: &Array2<i8>) -> Option<i8> {
        Some(1)
    }


}

pub struct TicTacToe {
    game_states: HashMap<Array2<i8>,TicTacToeState>
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            game_states: HashMap::new()
        }
    }
}