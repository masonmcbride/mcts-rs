use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::games::tictactoe::{TicTacToe, TicTacToeState};


pub struct MCTSNode {
    game_state: TicTacToeState,
    is_terminal: bool,
    is_expanded: bool,
    N: u32, // visit count
    Q: f64, // reguralized value
    children_and_edge_visits: Vec<MCTSNode, u32>,
    results = Hasmap<i32, u32> // {-1: num_losses, 0: num_draws, 1: num_wins}
}

impl MCTSNode {
    pub fn new(game_state: Rc<TicTacToeState>) -> MCTSNode {
        let is_terminal = game_state.is_terminal;
        MCTSNode {
            game_state,
            is_terminal,
            is_expanded: false,
            children_and_edge_visits: Vec::new(),
            N: 0.,
            Q: 0.,
            U: 0.,
        }
    }
}

pub struct MCTS {
    game: Rc<TicTacToe>,
    nodes: HashMap<Rc<TicTacToeState>,Rc<RefCell<MCTSNode>>>
}

impl MCTS {

    pub fn new(game: MCTSNode) -> Self {
        MCTS {
            game: game,
            nodes: HashMap::new(),
        }
    }

    pub fn select(self: &mut tree) -> Vec<MCTSNode> {
        
    }

    pub fn expand(self: &mut tree, path: Vec<MCTSNode>) {

    }

    pub fn rollout(self: &mut tree, MCTSNode) {

    }

    pub fn backprop(self: &mut tree, reward: i32) {

    }
    /*
    Each MCTS tree is a collection of MCTSNodes.
    Each MCTSNode represents a unique game state of the given game.
    So MCTS should AVOID creating new instances of the same MCTSNode, instead
    it should just point to the already created node. THis is called MCGS 

    pub fn search(self: &mut tree) {
        let path = tree.select();
        path = tree.expand(path);
        path = tree.rollout(path[-1]);
        self.backprop(path, reward);
    }
}