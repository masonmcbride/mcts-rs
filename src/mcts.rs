use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::games::tictactoe::{TicTacToe, TicTacToeState};


pub struct MCTSNode {
    game_state: Rc<TicTacToeState>,
    is_terminal: bool,
    is_expanded: bool,
    children_and_edge_visits: Vec<(Rc<RefCell<MCTSNode>>,f64)>,
    N: f64,
    Q: f64,
    U: f64,
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

    pub fn new(game: Rc<TicTacToe>) -> Self {
        MCTS {
            game: game,
            nodes: HashMap::new(),
        }

    }
    pub fn search(&mut self, node: Rc<RefCell<MCTSNode>>) {
        if node.borrow().is_terminal {
            // node.U = leaf node value
            node.borrow_mut().U = node.borrow().game_state.result.expect("Terminal state should have a reward").into();
        } else if node.borrow().N == 0. {
            // node.U = node rollout
            node.borrow_mut().U = self.rollout(node.clone()); // Q does the strong counter decrease after this goes out of scope
        } else {
            // node.U 
            

            /* 
            action = select_action_according_to_puct(node)
            if action not in node.children_and_edge_visits:
                new_game_state = node.game_state.play(action)
                if new_game_state.hash in nodes_by_hash:
                    child = nodes_by_hash[new_game_state.hash]
                    node.children_and_edge_visits[action] = (child,0)
                else:
                    new_node = Node(N=0,Q=0,game_state=new_game_state)
                    node.children_and_edge_visits[action] = (new_node,0)
                    nodes_by_hash[new_game_state.hash] = new_node
            (child,edge_visits) = node.children_and_edge_visits[action]
            perform_one_playout(child)
            node.children_and_edge_visits[action] = (child,edge_visits+1)
            */

           
        }
        let (total_edge_visits, weighted_q_sum) = {
            node.borrow().children_and_edge_visits.iter().fold(
                (0.0, 0.0),
                |(sum_visits, sum_q), (child, edge_visits)| {
                    (sum_visits + edge_visits, sum_q + (child.borrow().Q * edge_visits))
                },
            )
        };

        node.borrow_mut().N = 1. + total_edge_visits;
        node.borrow_mut().Q = (1./node.borrow().N) * (node.borrow().U + weighted_q_sum);

    }

    fn rollout(&self, node: Rc<RefCell<MCTSNode>>) -> f64 {
        1.
    }

}