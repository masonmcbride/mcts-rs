use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    hash::{Hash, Hasher},
};

/// -------------------------------------------------
/// Minimal TicTacToe definitions (stubs for example)
/// -------------------------------------------------

#[derive(Clone, Debug)]
pub struct TicTacToeState {
    /// This indicates if the game is over.
    pub is_terminal: bool,

    /// The current player: for example, +1 or -1.
    pub player: i32,

    // Add more state details as needed (e.g. board array).
}

/// Implement PartialEq, Eq, and Hash for TicTacToeState if you want
/// to store it in a HashMap. This is a naive example for demonstration.
/// You might replace or refine this with real equality/hashing logic.
impl PartialEq for TicTacToeState {
    fn eq(&self, other: &Self) -> bool {
        // Example: we consider them equal if both are terminal states
        // and have same player. Real logic should compare the entire board.
        self.is_terminal == other.is_terminal && self.player == other.player
    }
}

impl Eq for TicTacToeState {}

impl Hash for TicTacToeState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Example: combine is_terminal + player into the hash
        // In real code, you'd hash the board contents as well
        self.is_terminal.hash(state);
        self.player.hash(state);
    }
}

impl TicTacToeState {
    /// Return a list of all possible actions from this state.
    /// In TicTacToe, typical actions might be all empty cells.
    pub fn all_legal_actions(&self) -> Vec<i32> {
        // Stubbed out; fill with real logic.
        // For example, each action could be an index of the board.
        vec![0, 1, 2] // placeholder
    }

    /// Apply an action and get a new state.
    /// In real code, you would modify the board, switch the player, etc.
    pub fn transition(&self, _action: i32) -> TicTacToeState {
        // Stub. Replace with real transition logic.
        // For now, just clone and flip is_terminal if you like.
        let mut next_state = self.clone();
        // Example: just increment the player or do something arbitrary
        next_state.player = -self.player;
        next_state
    }
}

/// -------------------------------------------------
/// MCTS Node definition
/// -------------------------------------------------

#[derive(Debug)]
pub struct MCTSNode {
    /// Which game state does this node represent?
    pub game_state: TicTacToeState,

    /// Whether the node's state is terminal in the game sense.
    pub is_terminal: bool,

    /// Whether the node has been expanded (children discovered).
    pub is_expanded: bool,

    /// Total visit count.
    pub n: u32,

    /// Value estimate (could be average reward, Q-value, etc.)
    pub q: f64,

    /// Results for outcomes: e.g. -1 => losses, 0 => draws, 1 => wins.
    pub results: HashMap<i32, u32>,

    /// Each child node along with number of visits for the edge leading to it.
    pub children: RefCell<Vec<Rc<MCTSNode>>>,
    pub child_edge_visits: RefCell<HashMap<usize, u32>>, 
    // Alternatively: 
    // pub child_edge_visits: RefCell<HashMap<Rc<MCTSNode>, u32>>,
    // but that can be trickier because Rc doesn't implement Hash by default.
}

impl MCTSNode {
    pub fn new(game_state: TicTacToeState) -> Self {
        let is_terminal = game_state.is_terminal;
        let mut results = HashMap::new();
        // Initialize a small map for -1, 0, +1 outcomes
        results.insert(-1, 0);
        results.insert(0, 0);
        results.insert(1, 0);

        MCTSNode {
            game_state,
            is_terminal,
            is_expanded: false,
            n: 0,
            q: 0.0,
            results,
            children: RefCell::new(Vec::new()),
            child_edge_visits: RefCell::new(HashMap::new()),
        }
    }
}

/// -------------------------------------------------
/// MCTS definitions
/// -------------------------------------------------

pub struct MCTS {
    /// The root node of the MCTS tree
    pub root: Rc<MCTSNode>,

    /// A map from game states to nodes, so we don't create duplicates
    pub nodes: RefCell<HashMap<TicTacToeState, Rc<MCTSNode>>>,
}

impl MCTS {
    /// Create a new MCTS from an initial TicTacToe state.
    pub fn new(game_state: TicTacToeState) -> Self {
        let root_node = Rc::new(MCTSNode::new(game_state.clone()));
        let mut map = HashMap::new();
        map.insert(game_state, Rc::clone(&root_node));

        MCTS {
            root: root_node,
            nodes: RefCell::new(map),
        }
    }

    /// Return the MCTS node for the given state, creating it if necessary.
    pub fn get_node(&self, game_state: &TicTacToeState) -> Rc<MCTSNode> {
        if let Some(node) = self.nodes.borrow().get(game_state) {
            Rc::clone(node)
        } else {
            // create a new node
            let new_node = Rc::new(MCTSNode::new(game_state.clone()));
            self.nodes.borrow_mut().insert(game_state.clone(), Rc::clone(&new_node));
            new_node
        }
    }

    /// PUCT formula for picking the best child. This is a naive example.
    fn puct(&self, parent: &MCTSNode, child: &MCTSNode) -> f64 {
        let c_puct = 1.0;
        // We'll assume child_edge_visits is stored by index in parent's children
        // Or do something else if you prefer a map from Rc to visits.
        // For simplicity, let's just find the child's index in parent's children:
        let children = parent.children.borrow();
        let child_index = children.iter().position(|c| Rc::ptr_eq(c, &Rc::new(child.clone())));

        let n_sa = if let Some(idx) = child_index {
            *parent.child_edge_visits.borrow().get(&idx).unwrap_or(&1)
        } else {
            1
        };

        let parent_n = parent.n.max(1); // avoid div-by-zero
        child.q + c_puct * ((parent_n as f64).sqrt() / (1.0 + n_sa as f64))
    }

    /// Pick the best child of a node based on the PUCT value.
    pub fn best_child(&self, node: &MCTSNode) -> Option<Rc<MCTSNode>> {
        let children = node.children.borrow();
        children
            .iter()
            .max_by(|a, b| {
                let va = self.puct(node, &a);
                let vb = self.puct(node, &b);
                va.partial_cmp(&vb).unwrap()
            })
            .cloned()
    }

    /// Selection: go down the tree along best children until a leaf/terminal node is found.
    pub fn select_path(&self) -> Vec<Rc<MCTSNode>> {
        let mut path = vec![Rc::clone(&self.root)];

        loop {
            let last = path.last().unwrap();
            if !last.is_expanded || last.is_terminal {
                break;
            }
            if let Some(next_node) = self.best_child(last) {
                // increment edge visit count
                let idx = {
                    let children = last.children.borrow();
                    children.iter().position(|c| Rc::ptr_eq(c, &next_node))
                };
                if let Some(i) = idx {
                    let mut ce = last.child_edge_visits.borrow_mut();
                    *ce.entry(i).or_insert(0) += 1;
                }
                path.push(next_node);
            } else {
                break;
            }
        }
        path
    }

    /// Expansion: create children for the last node in the path if it's not terminal.
    pub fn expand(&self, path: &mut Vec<Rc<MCTSNode>>) {
        let last = path.last().unwrap();
        if last.is_terminal {
            return;
        }

        // Expand the node by creating children for every legal action
        let actions = last.game_state.all_legal_actions();
        for action in actions {
            let child_state = last.game_state.transition(action);
            let child_node = self.get_node(&child_state);

            // Add this child to the parent's children vector
            {
                let mut children = last.children.borrow_mut();
                let child_index = children.len();
                children.push(Rc::clone(&child_node));

                // Initialize the edge visits to 0 or 1, your choice
                last.child_edge_visits.borrow_mut().insert(child_index, 0);
            }
        }

        // Mark the node as expanded
        let node_mut = Rc::get_mut(path.last_mut().unwrap()).expect("Should be unique Rc here");
        node_mut.is_expanded = true;
    }

    /// Rollout: simulate a random (or heuristic-based) playout from a node to get a reward.
    /// In TicTacToe, you'd play until terminal and see if that results in a win/loss/draw.
    /// For demonstration, let's just return a dummy "reward distribution".
    pub fn rollout(&self, node: &MCTSNode) -> HashMap<i32, u32> {
        // You can implement actual logic. We'll do a dummy reward:
        let mut reward = HashMap::new();
        reward.insert(-1, 0);
        reward.insert(0, 0);
        reward.insert(1, 1);
        reward
    }

    /// Backprop: propagate the result back up the path.
    /// For demonstration, let's say we have a single "scalar reward"
    /// for the perspective of the path's initial player, etc.
    pub fn backprop(&self, path: &[Rc<MCTSNode>], outcome: &HashMap<i32, u32>) {
        // We'll interpret `outcome` as: if path's last player is +1 and outcome[1] = 1 => it's a win, etc.
        // In a real MCTS, you'd want a single scalar or something to pass upward.
        // This is just a dummy approach.
        let mut cur_reward: i32 = 1; // example; flip signs as you go up

        for node_rc in path.iter().rev() {
            // We need a mutable reference to the MCTSNode inside the Rc, 
            // which is typically done by storing RefCell<MCTSNode> or 
            // carefully reorganizing logic so we can mutate directly.
            // For simplicity, let's assume we only wanted to read or 
            // we store data in interior mutables. 
            // We’ll do something simplistic:

            let node = Rc::as_ptr(node_rc) as *mut MCTSNode;
            unsafe {
                // Increase the node's total visits
                (*node).n += 1;

                // Suppose the reward is from the perspective of each node's current player
                // We'll just do an example flipping sign:
                let r = if cur_reward > 0 { 1 } else { -1 } as i32;

                // Update Q (dummy approach; you may want average or something else)
                (*node).q += r as f64;

                // Update results 
                (*node).results.entry(r).and_modify(|val| *val += 1);

                // Flip reward for next parent up
                cur_reward = -cur_reward;
            }
        }
    }

    /// The main MCTS loop: select, expand, rollout, and backprop.
    pub fn run_once(&self) {
        // 1) Selection
        let mut path = self.select_path();

        // 2) Expansion
        self.expand(&mut path);

        // If the newly expanded node is not terminal, pick one new child or so
        // but that’s up to your approach. We will just pick the last node in the path.
        let leaf = path.last().unwrap();

        // 3) Rollout
        let reward = self.rollout(&leaf);

        // 4) Backprop
        self.backprop(&path, &reward);
    }
}

/// -------------------------------------------------
/// Example main function to show usage
/// ------------------------------------------------

fn main() {
    // Create a starting state for TicTacToe
    let initial_state = TicTacToeState {
        is_terminal: false,
        player: 1,
    };

    // Create our MCTS
    let mcts = MCTS::new(initial_state);

    // Run a few MCTS iterations
    for _ in 0..10 {
        mcts.run_once();
    }

    println!("MCTS finished running!");
}
