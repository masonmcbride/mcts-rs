use std::rc::Rc;
use ndarray::prelude::*;
use mcts_rs::games::connect4::{Connect4State, Connect4};
use mcts_rs::game::Game;

#[test] 
fn test_connect4_all_legal_actions() {
    let cant_lose = arr2(&[
        [ 0, -1,  0, -1,  1, -1,  0],
        [-1,  1,  0,  1, -1,  1, -1],
        [-1,  1,  1,  1, -1, -1,  1],
        [ 1, -1,  1, -1,  1, -1,  1],
        [ 1, -1,  1, -1,  1, -1,  1],
        [-1,  1, -1,  1, -1,  1, -1],
    ]);
    let mut connect4 = Connect4::new();
    let initial_state = connect4.get_state(&cant_lose);
    let expected_actions = vec![(0, 0), (1, 2), (0, 6)];

    assert_eq!(initial_state.all_legal_actions.clone().unwrap(), expected_actions,
        "The legal actions do not match the expected actions");
}
#[test]
fn test_connect4_finds_all_states() {
    fn explore_states(game: &mut Connect4, state: Rc<Connect4State>) {
        if !state.is_terminal {
            for action in state.all_legal_actions.clone().unwrap().iter() {
                let next_state = game.transition(Rc::clone(&state), *action);
                explore_states(game, next_state);
            }
        }
    }
    let cant_lose = arr2(&[
        [ 0, -1,  0, -1,  1, -1,  0],
        [-1,  1,  0,  1, -1,  1, -1],
        [-1,  1,  1,  1, -1, -1,  1],
        [ 1, -1,  1, -1,  1, -1,  1],
        [ 1, -1,  1, -1,  1, -1,  1],
        [-1,  1, -1,  1, -1,  1, -1],
    ]);
    let mut connect4 = Connect4::new();
    let initial_state = connect4.get_state(&cant_lose);
    explore_states(&mut connect4, initial_state);

    for (board, state) in connect4.game_states.iter() {
        assert_eq!(Rc::strong_count(state), 1 , "Strong count for state {:?} is not equal to 1", board);
    }

    // Assert that the number of states is exactly 5478
    assert_eq!(connect4.game_states.len(), 16, "The number of states should be 5478");
}