use std::collections::HashMap;
use std::rc::Rc;
use super::games::tictactoe::{TicTacToe, TicTacToeState};


pub struct MCTSNode {
    game_state: Rc<TicTacToeState>,
    is_terminal: bool,
    is_expanded: bool,
    N: u32, // visit count
    Q: f64, // reguralized value
    child_to_edge_visits: HashMap<Rc<MCTSNode>,u32>,
    results: HashMap<i32, u32> // {-1: num_losses, 0: num_draws, 1: num_wins}
}

impl MCTSNode {
    pub fn new(game_state: Rc<TicTacToeState>) -> MCTSNode {
        let terminal_result = game_state.is_terminal;
        MCTSNode {
            game_state: game_state,
            is_terminal: terminal_result,
            is_expanded: false,
            N: 0,
            Q: 0.,
            child_to_edge_visits: HashMap::new(),
            results: [(-1,0),(0,0),(1,0)].into_iter().collect()
        }
    }
}

pub struct MCTS {
    root: Rc<MCTSNode>,
    nodes: HashMap<Rc<TicTacToeState>,Rc<MCTSNode>>
}

impl MCTS {

    pub fn new(game_state: Rc<TicTacToeState>) -> Self {
        let mut mcts = MCTS {
            root: Rc::new(MCTSNode::new(Rc::clone(&game_state))),
            nodes: HashMap::new()
        };
        mcts.root = mcts.get_node(Rc::clone(&game_state));
        mcts
    }

    pub fn get_node(&mut self, game_state: Rc<TicTacToeState>) -> Rc<MCTSNode> {
        if let Some(node) = self.nodes.get(&game_state) {
            Rc::clone(node)
        } else {
            let new_node = Rc::new(MCTSNode::new(Rc::clone(&game_state)));
            self.nodes.insert(Rc::clone(&game_state), Rc::clone(&new_node));
            new_node
        }
    }

    pub fn best_child(&self, node: MCTSNode) -> MCTSNode {
        return max([child for child in node.children],key=lambda x: self.PUCT(node,x));
    }

    pub fn select(&self) -> Vec<Rc<MCTSNode>> {
        let n = path.len();
        let path = [Rc::clone(&self.root)];
        while path[-1].is_expanded && path.last().is_terminal {
            let next_node = self.best_child(path.last());
            path.last().child_to_edge_visits[next_node] += 1;
            path.append(next_node);
        }
        return path;
    }

    pub fn expand(&self, path: Vec<Rc<MCTSNode>>) -> Vec<Rc<MCTSNode>> {
        let expanding_node = path.last();
        if expanding_node.is_terminal {
            return path;
        } 
        game = expanidng_node.game_state;
        for action in game.all_legal_actions {
            child_node = self.get_node(game.transition(action));
            expanding_node.child_to_edge_visits[child_node] = 1
            if child_node.N == 0 {
                reward = self.rollout(child_node)
                self.backprop(path + [child_node], reward)
            }
        }
        expanding_node.is_expandied = True;
        return path.append(self.best_child(expanding_node));
    }

    pub fn rollout(&self, node: MCTSNode) -> () {
        return [(-1,1),(0,0),(1,0)].into_iter.collect();
    }

    pub fn backprop(&self, path: Vec<Rc<MCTSNode>>, reward: HashMap<i32,u32>) {
        let reward = reward[path[-1].game_state.player];
        for node in path.iter().rev() {
            node.N = 1 + sum(node.child_to_edge_visits.values());
            node.Q = -(1/node.N)*(reward + sum(child.Q*edge_vists for (child,edge_vists) in node.child_to_edge_visits.items()));
            node.results[reward] += 1;
            reward = -reward;
        }
    }

    pub fn run(&self) {
        let path = self.select();
        path = self.expand(path);
        path = self.rollout(path[-1]);
        self.backprop(path, reward);
    }

    pub fn PUCT(&self, parent: Rc<MCTSNode>, node: Rc<MCTSNode>) -> f64 {
        let c_puct = 1.;
        let N_sa = parent.child_to_edge_visits[node];
        return node.Q + c_puct * 1 * np.sqrt(parent.N) / (1 + N_sa);
    }
}