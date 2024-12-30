use std::{collections::HashMap, hash::Hash};
use std::rc::Rc;
use super::games::tictactoe::{TicTacToe, TicTacToeState};


pub struct MCTSNode {
    is_terminal: bool,
    is_expanded: bool,
    N: u32, // visit count
    Q: f64, // reguralized value
    results: HashMap<i32, u32> // {-1: num_losses, 0: num_draws, 1: num_wins}
}

impl MCTSNode {
    pub fn new(game_state: TicTacToeState) -> MCTSNode {
        let is_terminal = game_state.is_terminal;
        MCTSNode {
            is_terminal,
            is_expanded: false,
            N: 0,
            Q: 0.,
            results: [(-1,0),(0,0),(1,0)].into_iter().collect()
        }
    }
}

pub struct MCTS {
    root: Rc<MCTSNode>,
    nodes: HashMap<Rc<TicTacToeState>,Rc<MCTSNode>>
}

impl MCTS {

    pub fn new(game_state: TicTacToeState) -> Self {
        let root = MCTSNode::new(game_state);
        MCTS {
            root: &root,
            nodes: [(game_state, root)].into_iter().collect()
        }
    }

    pub fn select(self: &tree) -> Vec<MCTSNode> {
        let path = [Rc::clone(tree.root)];
        while path.last().is_expanded and 
    }

    pub fn expand(self: &tree, path: Vec<MCTSNode>) -> Vec<MCTSNode> {
        return Vec::new();
    }

    pub fn rollout(self: &tree, MCTSNode) -> () {
        ()
    }

    pub fn backprop(self: &tree, path: Vec<Rc<RefCell<MCTSNode>>>, reward: i32) {
        for node in path.iter().rev() {
            let mut node = node.borrow_mut();
            node.N += 1;
        }
    }

    pub fn search(self: &tree) {
        let path = tree.select();
        path = tree.expand(path);
        path = tree.rollout(path[-1]);
        self.backprop(path, reward);
    }
}