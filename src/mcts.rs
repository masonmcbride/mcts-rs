use std::collections::HashMap;
use crate::games::GameState;

pub struct MCTS<G: GameState> {
    root: MCTSNode<G>,
    nodes: HashMap<G, MCTSNode<G>>
}

impl<G: GameState> MCTS<G> {
    pub fn new(game_state: G) -> MCTS<G> {
        let root = MCTSNode::new(game_state);
        let mut nodes: HashMap::new();
        nodes.insert(&root.game_state, root);

        MCTS { root, nodes } 

    }
}

pub struct MCTSNode<G: GameState> {
    game_state: G,
    is_terminal: bool,
    is_expanded: bool,
    children: Vec<MCTSNode<G>>,
    N: u32,
    W: u32,
    P: f64,
    results: HashMap<i8, u32>
}

impl<G: GameState> MCTSNode<G> {
    pub fn new(game_state: G) -> MCTSNode<G> {
        let is_terminal = game_state.is_terminal();
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