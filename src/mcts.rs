use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use wyhash2::WyHash;
use crate::game::{Game,GameState};

pub struct MCTSNode<S: GameState> {
    pub game_state: Rc<S>,
    is_terminal: bool,
    is_expanded: bool,
    pub N: u32, // visit count
    pub Q: f64, // reguralized value
    pub child_to_edge_visits: HashMap<Rc<S>,u32,WyHash>,
    pub results: HashMap<i32, u32> // {-1: num_losses, 0: num_draws, 1: num_wins}
}

impl<S: GameState> MCTSNode<S> {
    pub fn new(game_state: Rc<S>) -> MCTSNode<S> {
        let terminal_result = *game_state.is_terminal();
        MCTSNode {
            game_state: game_state,
            is_terminal: terminal_result,
            is_expanded: false,
            N: 0,
            Q: 0.,
            child_to_edge_visits: HashMap::with_hasher(WyHash::with_seed(0)),
            results: [(-1,0),(0,0),(1,0)].into_iter().collect()
        }
    }
}

pub struct MCTS<G: Game> {
    pub root: Rc<RefCell<MCTSNode<G::State>>>,
    pub nodes: HashMap<Rc<G::State>,Rc<RefCell<MCTSNode<G::State>>>,WyHash>,
    pub game: G
}

impl<G: Game> MCTS<G> {

    pub fn new(game: G, root_state: Rc<G::State>) -> Self {
        let mut mcts = MCTS {
            root: Rc::new(RefCell::new(MCTSNode::new(root_state.clone()))),
            nodes: HashMap::with_hasher(WyHash::with_seed(0)),
            game: game
        };
        mcts.root = mcts.get_node(root_state);
        mcts
    }

    pub fn get_node(&mut self, game_state: Rc<G::State>) -> Rc<RefCell<MCTSNode<G::State>>> {
        if let Some(node) = self.nodes.get(&game_state) {
            node.clone()
        } else {
            let new_node = Rc::new(RefCell::new(MCTSNode::new(game_state.clone())));
            self.nodes.insert(game_state, new_node.clone());
            new_node
        }
    }

    pub fn select(&mut self) -> Vec<Rc<RefCell<MCTSNode<G::State>>>> {
        let mut path = vec![self.root.clone()];

        loop {
            let last_node_rc = path.last().unwrap().clone();
            let last_node = last_node_rc.borrow();
            if !last_node.is_expanded || last_node.is_terminal {
                break;
            }
            drop(last_node);

            let next_node_rc = self.best_child(last_node_rc.clone());
            let next_state = next_node_rc.borrow().game_state.clone();
            *last_node_rc.borrow_mut().child_to_edge_visits.get_mut(&next_state)
                          .expect("No edge visit entry?") += 1;
            path.push(next_node_rc);
        }
        path
    }

    pub fn expand(&mut self, mut path: Vec<Rc<RefCell<MCTSNode<G::State>>>>) -> Vec<Rc<RefCell<MCTSNode<G::State>>>> {
        let expanding_node_rc= path.last().unwrap().clone();
        if expanding_node_rc.borrow().is_terminal {
            return path;
        }
        let actions = expanding_node_rc.borrow().game_state.all_legal_actions().clone().unwrap();
        let mut child_nodes_to_backprop = Vec::new();

        {
            let mut node_mut = expanding_node_rc.borrow_mut();
            for action in &actions {
                let child_state = self.game.transition(node_mut.game_state.clone(), *action);
                node_mut.child_to_edge_visits.insert(child_state.clone(), 1);
                let child_node_rc = self.get_node(child_state);

                // Collect child nodes that need backprop
                if child_node_rc.borrow().N == 0 {
                    child_nodes_to_backprop.push(child_node_rc);
                }
            }
            node_mut.is_expanded = true;
        }

        for child_node_rc in child_nodes_to_backprop {
            let reward_map = self.rollout(child_node_rc.clone());
            let mut temp_path = path.clone();
            temp_path.push(child_node_rc);
            self.backprop(temp_path, reward_map);
        }

        path.push(self.best_child(expanding_node_rc));
        path
    }

    pub fn rollout(&mut self, node_rc: Rc<RefCell<MCTSNode<G::State>>>) -> HashMap<i32, i32> {
        let mut rng = thread_rng();

        let mut cur_state = node_rc.borrow().game_state.clone();
        while !cur_state.is_terminal() {
            let actions_vec = cur_state.all_legal_actions().clone().unwrap();
            let action = *actions_vec.choose(&mut rng).unwrap();
            cur_state = self.game.transition(cur_state, action);
        }

        let reward_map = cur_state.result().clone().expect("No result for terminal state?").into_iter().collect();
        reward_map
    }

    pub fn backprop(&mut self, path: Vec<Rc<RefCell<MCTSNode<G::State>>>>, reward_map: HashMap<i32,i32>) {
        if path.is_empty() {
            return;
        }

        let mut reward = *reward_map.get(&path.last().expect("backpropping on empty path?")
                                                   .borrow().game_state.player())
                                         .expect("Reward map is broken");
        for node_rc in path.into_iter().rev() {
            let sum_of_child_q_times_visits: f64 = {
                let node_borrow = node_rc.borrow();
                node_borrow
                    .child_to_edge_visits
                    .iter()
                    .map(|(child_state, &edge_visits)| {
                        let child_node_rc = self.get_node(child_state.clone());
                        let child_node_borrow = child_node_rc.borrow();
                        child_node_borrow.Q * edge_visits as f64
                    })
                    .sum()
            };
            let mut node_mut = node_rc.borrow_mut();
            node_mut.N = 1 + node_mut.child_to_edge_visits.values().sum::<u32>();
            node_mut.Q = -(1./node_mut.N as f64)*(reward as f64 + sum_of_child_q_times_visits);
            node_mut.results.entry(reward).and_modify(|n| {*n += 1});
            reward = -1 * reward;
        }
    }

    pub fn PUCT(&mut self, parent: Rc<RefCell<MCTSNode<G::State>>>, node: Rc<RefCell<MCTSNode<G::State>>>) -> f64 { 
        let parent_borrow = parent.borrow();
        let node_borrow = node.borrow();
        let c_puct = 1.;
        let N_sa = parent_borrow.child_to_edge_visits.get(&node_borrow.game_state).expect("Calling PUCT on a Node that doesn't exist?");
        return node_borrow.Q + c_puct * 1. * f64::sqrt(parent_borrow.N as f64) / (1 + N_sa) as f64;
    }

    pub fn best_child(&mut self, node: Rc<RefCell<MCTSNode<G::State>>>) -> Rc<RefCell<MCTSNode<G::State>>> {
        let children_states: Vec<Rc<G::State>> = {
            let node_borrow = node.borrow();
            node_borrow.child_to_edge_visits.keys().cloned().collect()
        };

        let best_child_state = children_states
            .into_iter()
            .max_by(|state_a, state_b| {
                let child_a = self.get_node(state_a.clone());
                let child_b = self.get_node(state_b.clone());
                let puct_a: f64 = self.PUCT(node.clone(),child_a);
                let puct_b: f64 = self.PUCT(node.clone(), child_b);
                puct_a.partial_cmp(&puct_b).expect("Comparison failed due to NaN")
            }).expect("Called best child on no children");
        
        self.get_node(best_child_state)
    }

    pub fn run(&mut self) {
        let mut path = self.select();
        path = self.expand(path);
        let reward = self.rollout(path.last().expect("Path is somehow empty").clone());
        self.backprop(path, reward);
    }

    pub fn search(&mut self, n: u32) {
      for _ in 0..n { self.run() }  
    }
}