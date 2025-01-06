use std::rc::Rc;
use ndarray::{Array1,Array2};
use std::hash::Hash;

pub trait GameState: PartialEq + Eq + Hash {
    fn state(&self) -> &Array2<i8>;
    fn is_terminal(&self) -> &bool;
    fn player(&self) -> &i32;
    fn result(&self) -> &Option<Vec<(i32,i32)>>;
    fn all_legal_actions(&self) -> &Array1<(usize, usize)>;
}

pub trait Game {
    type State: GameState;
    fn get_state(&mut self, board: &Array2<i8>) -> Rc<Self::State>;
    fn transition(&mut self, game_state: Rc<Self::State>, action: (usize,usize)) -> Rc<Self::State>;
}