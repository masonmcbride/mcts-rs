use ndarray::prelude::*;
use mcts_rs::game::Game;
use mcts_rs::games::tictactoe::TicTacToe;
use mcts_rs::mcts::MCTS;

#[test]
fn test_mcts_picks_winning_move_when_almost_won() {
    let mut tictactoe = TicTacToe::new();
    let one_move_to_win = arr2(&[
        [ 1, -1,  0],
        [ 1,  1, -1],
        [-1,  0,  0]]);
    let almost_won = tictactoe.get_state(&one_move_to_win);
    let mut mcts = MCTS::new(tictactoe,almost_won);
    mcts.search(10);

    let child_states = {
        let root = mcts.root.borrow();
        root.child_to_edge_visits
            .keys()
            .cloned() 
            .collect::<Vec<_>>()
    };

    let child_nodes = child_states
        .into_iter()
        .map(|child_state_rc| mcts.get_node(child_state_rc))
        .collect::<Vec<_>>();

    let winning_node = child_nodes
        .into_iter()
        .max_by(|a, b| {
            let a_q = a.borrow().Q;
            let b_q = b.borrow().Q;
            a_q.partial_cmp(&b_q).unwrap()
        }).expect("No child found");

    let winning_child_state = &winning_node.borrow().game_state.state;
    assert_eq!(winning_child_state[[2, 2]], 1, "MCTS did not pick the winning move.");
}

#[test]
fn test_mcts_results_contain_no_losses() {
    let mut tictactoe = TicTacToe::new();
    let one_move_to_win = arr2(&[
        [ 1, -1,  0],
        [ 1,  1, -1],
        [-1,  0,  0],
    ]);
    let almost_won = tictactoe.get_state(&one_move_to_win);
    let mut mcts = MCTS::new(tictactoe,almost_won);
    mcts.search(50);

    let root = mcts.root.borrow();
    let losses_for_o = *root.results.get(&-1).expect("results broken");
    assert_eq!(losses_for_o, 0 ,"Expected zero losses for player -1 at the root");
}

#[test]
fn test_mcts_blocks_win() {
    let mut tictactoe = TicTacToe::new();
    let board = arr2(&[
        [-1,  1,  0],
        [ 1, -1,  0],
        [ 0,  0,  0],
    ]);
    let o_can_win = tictactoe.get_state(&board);
    let blocked = arr2(&[
        [-1,  1,  0],
        [ 1, -1,  0],
        [ 0,  0,  1],
    ]);
    let mut mcts = MCTS::new(tictactoe,o_can_win);
    mcts.search(50);

    let child_states = {
        let root = mcts.root.borrow();
        root.child_to_edge_visits
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    };

    let child_nodes = child_states
        .into_iter()
        .map(|child_state_rc| mcts.get_node(child_state_rc.clone()))
        .collect::<Vec<_>>();

    let chosen_node = child_nodes
        .into_iter()
        .max_by(|a, b| {
            let a_q = a.borrow().Q;
            let b_q = b.borrow().Q;
            a_q.partial_cmp(&b_q).unwrap()
        })
        .expect("No child found");

    let chosen_state = &chosen_node.borrow().game_state.state;
    assert_eq!(*chosen_state, blocked, "MCTS did not block the winning move");
}

#[test]
fn test_one_run_expands_and_selects_one() {
    let mut tictactoe = TicTacToe::new();
    let empty_board = Array2::zeros((3, 3));
    let new_game = tictactoe.get_state(&empty_board);
    let mut mcts = MCTS::new(tictactoe, new_game);
    mcts.run();
    assert_eq!(mcts.root.borrow().N, 10, "One run visits the root and all it's children. 1 + 9 = 10 = root.N");
    mcts.run();
    assert_eq!(mcts.root.borrow().N, 11, "one more run has only one path up to root so + 1 more");
}