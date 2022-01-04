pub mod state_value_cache;
pub mod state_dependency_graph;
// system dynamics equation

// cost function

// state value cache

use std::collections::VecDeque;
use std::collections::HashMap;
use state_value_cache::StateValueCache;



pub fn solve_top_down<S,A>(
    x_t: fn(&S, &A) -> S,
    c: fn(&S) -> f64,
    actions_for_state: fn(&S) -> Vec<A>,
    cache_value: fn(&S, f64) -> (),
    lookup_value: fn(&S) -> Option<&f64>,
    x0: S,
) {
    
}


pub fn solve_bottom_up<S, A>(
    x_t: fn(&S) -> Vec<S>,
    c: fn(&S) -> f64,
    cache_value: fn(&S, f64) -> (),
    lookup_value: fn(&S) -> Option<&f64>,
    terminal_states: Vec<S>,
) {
    let mut states_remaining: VecDeque<S> = VecDeque::from(terminal_states);
    let maybe_working_state = states_remaining.pop_front();

    while let Some(working_state) = &maybe_working_state {
        let next_states = x_t(&working_state);


    } 
}