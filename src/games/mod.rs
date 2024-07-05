use ndarray::prelude::*;
pub mod tictactoe;
pub trait GameState: Sized + std::hash::Hash + Eq {
    fn state(&self) -> &Array2<i8>;
    fn is_terminal(&self) -> bool;
    fn player(&self) -> i8;
    fn result(&self) -> Option<i8>;
    fn all_legal_actions(&self) -> &Array1<(usize, usize)>;
}