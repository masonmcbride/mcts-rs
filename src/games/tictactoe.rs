use std::collections::HashMap;
use std::fmt;
use ndarray::prelude::*;

pub struct TicTacToe<'a> {
    pub game_states: HashMap<Array2<i8>, &'a TicTacToeState>
}

impl<'a> TicTacToe<'a> {
    pub fn new() -> Self {
        TicTacToe { game_states: HashMap::new() }
    }

    pub fn get_state(&mut self, board: &Array2<i8>) -> &'a TicTacToeState {
        self.game_states.entry(board.clone()).or_insert_with(|| {
            let new_state = TicTacToeState::new(board.clone());
            Box::leak(Box::new(new_state))
        })
    }

    pub fn transition(&mut self, game_state: &'a TicTacToeState, action: (usize, usize)) -> &'a TicTacToeState {
        let mut new_state = game_state.state.clone();
        new_state[action] = game_state.player;
        self.get_state(&new_state)
    }

}
//#[derive(Debug,std::hash::Hash,PartialEq,Eq)]
#[derive(Debug)]
pub struct TicTacToeState {
    state: Array2<i8>,
    player: i8,
    result: Option<i8>,
    is_terminal: bool,
    pub all_legal_actions: Array1<(usize,usize)>
}

impl TicTacToeState {
    pub fn new(state: Array2<i8>) -> TicTacToeState {
        let player: i8 = if state.sum() <= 0 { 1 } else { -1 };
        let result: Option<i8> = TicTacToeState::game_result(&state);
        let is_terminal: bool = result.is_some();
        let all_legal_actions:Array1<(usize, usize)> = if !is_terminal {
            state.indexed_iter()
                .filter(|&((_,_),&value)| value == 0)
                .map(|((i,j),_)| (i,j))
                .collect::<Array1<(usize,usize)>>()
        } else {
            Array1::from_shape_vec(0, Vec::new()).unwrap() // like wtf is this !! 
        };
        TicTacToeState {
            state,
            player,
            result,
            is_terminal,
            all_legal_actions
        }
}
    #[inline]
    fn game_result(state: &Array2<i8>) -> Option<i8> {
        let three_in_a_row = 3;
        let rowsum = state.sum_axis(Axis(0));
        let colsum = state.sum_axis(Axis(1));
        let diag_sum_tl = state.diag().sum();
        let diag_sum_tr = state.slice(s![..,..;-1]).diag().sum();

        let player_one_wins = rowsum.iter().any(|&x| x == three_in_a_row)
        || colsum.iter().any(|&x| x == three_in_a_row)
        || diag_sum_tl == three_in_a_row
        || diag_sum_tr == three_in_a_row;

        if player_one_wins {
            return Some(1);
        }

        let player_two_wins = rowsum.iter().any(|&x| x == -three_in_a_row)
        || colsum.iter().any(|&x| x == -three_in_a_row)
        || diag_sum_tl == -three_in_a_row
        || diag_sum_tr == -three_in_a_row;

        if player_two_wins {
            return Some(-1);
        }

        if state.iter().all(|&x| x != 0) {
            return Some(0)
        }

        None
    }


}

impl fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

/* 
impl GameState for TicTacToeState {
    fn state(&self) -> &Array2<i8> {
        &self.state
    }
    fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    fn player(&self) -> i8 {
        self.player
    }

    fn result(&self) -> Option<i8> {
        self.result
    }

    fn all_legal_actions(&self) -> &Array1<(usize, usize)> {
        &self.all_legal_actions
    }
}
*/