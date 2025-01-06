use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use ndarray::{Array1, Array2, Axis, s};
use crate::game::{Game,GameState};

pub struct TicTacToe {
    pub game_states: HashMap<Array2<i8>, Rc<TicTacToeState>>
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe { game_states: HashMap::new() }
    }
}

impl Game for TicTacToe {
    type State = TicTacToeState;

    fn get_state(&mut self, board: &Array2<i8>) -> Rc<TicTacToeState> {
        if let Some(state) = self.game_states.get(board) {
            state.clone()
        } else {
            let state_rc = Rc::new(TicTacToeState::new(board.clone()));
            self.game_states.insert(board.clone(), state_rc.clone());
            state_rc
        }
    }

    fn transition(&mut self, game_state: Rc<TicTacToeState>, action: (usize, usize)) -> Rc<TicTacToeState> {
        let mut new_state = game_state.state.clone();
        new_state[action] = game_state.player as i8;
        self.get_state(&new_state)
    }
}

#[derive(Debug,PartialEq,Eq,std::hash::Hash)]
pub struct TicTacToeState {
    pub state: Array2<i8>,
    pub player: i32,
    pub result: Option<Vec<(i32,i32)>>,
    pub is_terminal: bool,
    pub all_legal_actions: Array1<(usize,usize)>
}

impl TicTacToeState {
    pub fn new(state: Array2<i8>) -> TicTacToeState {
        let player = if state.sum() <= 0 { 1 } else { -1 };
        let result = TicTacToeState::game_result(&state);
        let is_terminal = result.is_some();
        let all_legal_actions= if !is_terminal {
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
    fn game_result(state: &Array2<i8>) -> Option<Vec<(i32,i32)>> {
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
            return Some([(1,1),(-1,-1)].to_vec());
        }

        let player_two_wins = rowsum.iter().any(|&x| x == -three_in_a_row)
        || colsum.iter().any(|&x| x == -three_in_a_row)
        || diag_sum_tl == -three_in_a_row
        || diag_sum_tr == -three_in_a_row;

        if player_two_wins {
            return Some([(1,-1),(-1,1)].to_vec());
        }

        if state.iter().all(|&x| x != 0) {
            return Some([(1,0),(-1,0)].to_vec());
        }

        None
    }
}

impl Display for TicTacToeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.state)
    }
}

impl GameState for TicTacToeState {
    fn state(&self) -> &Array2<i8> {
        &self.state
    }
    fn is_terminal(&self) -> &bool {
        &self.is_terminal
    }

    fn player(&self) -> &i32 {
        &self.player
    }

    fn result(&self) -> &Option<Vec<(i32,i32)>> {
        &self.result
    }

    fn all_legal_actions(&self) -> &Array1<(usize, usize)> {
        &self.all_legal_actions
    }
}