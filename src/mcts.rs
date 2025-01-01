use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use rand::seq::SliceRandom;
use rand::thread_rng;
use super::games::tictactoe::{TicTacToe,TicTacToeState};

pub struct MCTSNode {
    pub game_state: Rc<TicTacToeState>,
    is_terminal: bool,
    is_expanded: bool,
    N: u32, // visit count
    Q: f64, // reguralized value
    child_to_edge_visits: HashMap<Rc<TicTacToeState>,u32>,
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

// TicTacToeState already is hashable, and the requirement for equality in MCTSNode is 
// if the TicTacToeStates they represent are the same. So just pass to that check. 
// TODO: maybe there is an even better way, but this is nice. 
impl PartialEq for MCTSNode {
    fn eq(&self, other: &Self) -> bool {
        self.game_state == other.game_state
    }
}

impl Eq for MCTSNode {} 

impl Hash for MCTSNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.game_state.hash(state)
    }    
}


pub struct MCTS {
    pub root: Rc<RefCell<MCTSNode>>,
    pub nodes: HashMap<Rc<TicTacToeState>,Rc<RefCell<MCTSNode>>>,
    pub tictactoe: TicTacToe
}

impl MCTS {

    pub fn new(game_state: Rc<TicTacToeState>, game: TicTacToe) -> Self {
        let mut mcts = MCTS {
            root: Rc::new(RefCell::new(MCTSNode::new(Rc::clone(&game_state)))),
            nodes: HashMap::new(),
            tictactoe: game
        };
        mcts.root = mcts.get_node(Rc::clone(&game_state));
        mcts
    }

    pub fn get_node(&mut self, game_state: Rc<TicTacToeState>) -> Rc<RefCell<MCTSNode>> {
        if let Some(node) = self.nodes.get(&game_state) {
            Rc::clone(node)
        } else {
            let new_node = Rc::new(RefCell::new(MCTSNode::new(Rc::clone(&game_state))));
            self.nodes.insert(Rc::clone(&game_state), Rc::clone(&new_node));
            new_node
        }
    }

    pub fn PUCT(&mut self, parent: Rc<RefCell<MCTSNode>>, node: Rc<RefCell<MCTSNode>>) -> f64 { 
        let parent_borrow = parent.borrow();
        let node_borrow = node.borrow();
        let c_puct = 1.;
        let N_sa = parent_borrow.child_to_edge_visits.get(&node_borrow.game_state).expect("Calling PUCT on a Node that doesn't exist?");
        return node_borrow.Q + c_puct * 1. * f64::sqrt(parent_borrow.N as f64) / (1 + N_sa) as f64;
    }

    pub fn best_child(&mut self, node: Rc<RefCell<MCTSNode>>) -> Rc<RefCell<MCTSNode>> {
        let children_states: Vec<Rc<TicTacToeState>> = {
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
        
        Rc::clone(self.nodes.get(&best_child_state).expect("Child node not in map?"))
    }

    pub fn select(&mut self) -> Vec<Rc<RefCell<MCTSNode>>> {
        let mut path = vec![Rc::clone(&self.root)];
        
        loop {
            let last_node_rc = match path.last() {
                Some(node) => Rc::clone(node),
                None => break,
            };
            let last_node = last_node_rc.borrow();
            if !last_node.is_expanded || last_node.is_terminal {
                break;
            }

            let next_node_rc = self.best_child(Rc::clone(&last_node_rc));
            drop(last_node);

            {
                let mut last_node_mut = last_node_rc.borrow_mut();
                let child_state = next_node_rc.borrow().game_state.clone();
                *last_node_mut.child_to_edge_visits.get_mut(&child_state)
                .expect("No edge visit entry?") += 1;
            }

            path.push(next_node_rc);
        }
        path
    }

    pub fn expand(&mut self, mut path: Vec<Rc<RefCell<MCTSNode>>>) -> Vec<Rc<RefCell<MCTSNode>>> {
        let expanding_node_rc= match path.last() {
            Some(node) => Rc::clone(node),
            None => return path,
        };

        {
            let mut node_mut = expanding_node_rc.borrow_mut();
            if node_mut.is_terminal {
                return path;
            }

            let actions = &expanding_node_rc.borrow().game_state.all_legal_actions;
            for action in actions {
                let child_state = self.tictactoe.transition(node_mut.game_state.clone(), *action);
                node_mut.child_to_edge_visits.entry(Rc::clone(&child_state)).or_insert(0);

                let child_node_rc = self.get_node(child_state);
                if child_node_rc.borrow().N == 0 {
                    let reward_map = self.rollout(Rc::clone(&child_node_rc));
                    let mut temp_path = path.clone();
                    temp_path.push(Rc::clone(&child_node_rc));
                    self.backprop(temp_path, reward_map);
                }
            }
            node_mut.is_expanded = true;
        }

        let next_node_rc = self.best_child(Rc::clone(&expanding_node_rc));
        path.push(next_node_rc);
        path
    }

    pub fn rollout(&mut self, node_rc: Rc<RefCell<MCTSNode>>) -> HashMap<i32, i32> {
        let mut rng = thread_rng();

        let mut cur_state = node_rc.borrow().game_state.clone();

        while !cur_state.is_terminal {
            let actions_vec = cur_state.all_legal_actions.clone().into_raw_vec();
            if actions_vec.is_empty() {
                break;
            }

            let action = *actions_vec.choose(&mut rng).unwrap();
            cur_state = self.tictactoe.transition(cur_state, action);
        }

        let final_result = cur_state.result.unwrap_or(0);
        let mut reward_map: HashMap<i32, i32> = [(-1,1),(0,0),(1,0)].into_iter().collect();
        reward_map.insert(final_result as i32, 1);
        reward_map
    }

    pub fn backprop(&self, path: Vec<Rc<RefCell<MCTSNode>>>, reward_map: HashMap<i32,i32>) {
        if path.is_empty() {
            return;
        }

        let last_node_rc = path.last().unwrap();
        let last_player = last_node_rc.borrow().game_state.player;

        let mut reward = *reward_map.get(&last_player).expect("Reward map is broken");
        for node_rc in path.into_iter().rev() {
            let mut node_mut = node_rc.borrow_mut();
            node_mut.N = 1 + node_mut.child_to_edge_visits.values().sum::<u32>();
            let sum_of_child_q_times_visits: f64 = node_mut
                .child_to_edge_visits
                .iter()
                .map(|(child_state, &edge_visits)| {
                    let child_node_rc = self
                        .nodes.get(child_state).expect("No child in MCTS::nodes?");
                    let child_node_borrow = child_node_rc.borrow();
                    child_node_borrow.Q * edge_visits as f64
                }).sum();
            let n_f = node_mut.N as f64;
            node_mut.Q = -(1./n_f)*(reward as f64 + sum_of_child_q_times_visits);
            *node_mut.results.entry(reward).or_insert(0) += 1;
            reward = -1 * reward;
        }
    }

    pub fn run(&mut self) {
        let mut path = self.select();
        path = self.expand(path);
        let reward = self.rollout(path.last().expect("Path is somehow empty").clone());
        self.backprop(path, reward);
    }

}