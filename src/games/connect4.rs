use std::collections::HashMap;
use std::fmt::{Display,Formatter,Result};
use std::rc::Rc;
use ndarray::{Array2,Axis,s};
use wyhash2::WyHash;
use crate::game::{Game,GameState};

pub struct Connect4 {
    pub game_states: HashMap<Array2<i8>, Rc<Connect4State>,WyHash>
}

impl Connect4 {
    pub fn new() -> Self {
        Connect4 { game_states: HashMap::with_hasher(WyHash::with_seed(0)) }
    }
}

impl Game for Connect4 {
    type State = Connect4State;

    fn get_state(&mut self, board: &Array2<i8>) -> Rc<Connect4State> {
        if let Some(state) = self.game_states.get(board) {
            state.clone()
        } else {
            let state_rc = Rc::new(Connect4State::new(board.clone()));
            self.game_states.insert(board.clone(), state_rc.clone());
            state_rc
        }
    }

    fn transition(&mut self, game_state: Rc<Connect4State>, action: (usize, usize)) -> Rc<Connect4State> {
        let mut new_state = game_state.state.clone();
        new_state[action] = game_state.player as i8;
        self.get_state(&new_state)
    }
}

#[derive(Debug,PartialEq,Eq,std::hash::Hash)]
pub struct Connect4State {
    pub state: Array2<i8>,
    pub player: i32,
    pub result: Option<Vec<(i32,i32)>>,
    pub is_terminal: bool,
    pub all_legal_actions: Option<Vec<(usize,usize)>>
}

impl Connect4State {
    pub fn new(state: Array2<i8>) -> Connect4State {
        let player = if state.sum() <= 0 { 1 } else { -1 };
        let result = Connect4State::game_result(&state);
        let is_terminal = result.is_some();
        let all_legal_actions = {
            let mut actions = Vec::new();
            for j in 0..state.ncols() {
                if let Some(i) = (0..state.nrows()).rev().find(|&i| state[[i,j]] == 0) {
                    actions.push((i,j));
                }
            }
            Some(actions)
        };
        Connect4State {
            state,
            player,
            result,
            is_terminal,
            all_legal_actions
        }
    }
    
    #[inline]
    fn game_result(state: &Array2<i8>) -> Option<Vec<(i32,i32)>> {
        let mut has_empty_cells = false;

        for i in 0..state.nrows() - 3 {
            for j in 0..state.ncols() - 3 {
                let view = state.slice(s![i..i + 4, j..j + 4]);
                let horiz_sum = view.sum_axis(Axis(1));
                let vert_sum = view.sum_axis(Axis(0));
                let diag_sum_tl = view.diag().sum();
                let diag_sum_tr = view.slice(s![.., ..;-1]).diag().sum();

                if horiz_sum.iter().any(|&x| x == 4) || vert_sum.iter().any(|&x| x == 4) || diag_sum_tl == 4 || diag_sum_tr == 4 {
                    return Some(vec![(1, 1), (-1, -1)]);
                }

                if horiz_sum.iter().any(|&x| x == -4) || vert_sum.iter().any(|&x| x == -4) || diag_sum_tl == -4 || diag_sum_tr == -4 {
                    return Some(vec![(1, -1), (-1, 1)]);
                }

                if view.iter().any(|&x| x == 0) {
                    has_empty_cells = true;
                }
            }
        }

        if !has_empty_cells {
            return Some(vec![(1, 0), (-1, 0)]);
        }

        None
    }
}

impl Display for Connect4State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.state)
    }
}

impl GameState for Connect4State {
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

    fn all_legal_actions(&self) -> &Option<Vec<(usize, usize)>> {
        &self.all_legal_actions
    }
}