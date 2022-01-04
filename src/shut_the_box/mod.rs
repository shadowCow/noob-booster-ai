use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    dice_value: u32,
    tiles_open: [bool; 10],
}

impl State {
    pub fn initial() -> Vec<State> {
        let mut states: Vec<State> = vec![];

        for i in 2..=12 {
            states.push(State::fresh(i));
        }
    
        states
    }

    pub fn fresh(dice_value: u32) -> State {
        State {
            dice_value,
            tiles_open: [true; 10],
        }
    }

    pub fn from(
        dice_value: u32,
        tiles_open: &[bool; 10],
    ) -> State {
        State {
            dice_value,
            tiles_open: *tiles_open,
        }
    }

    pub fn rolls_for(tiles_open: &[bool; 10]) -> Vec<State> {
        let mut states: Vec<State> = vec![];

        for i in 2..=12 {
            states.push(State::from(i, tiles_open));
        }

        states
    }

    pub fn score(&self) -> u32 {
        let mut sum: u32 = 0;

        for v in 0..self.tiles_open.len() {
            if self.tiles_open[v] {
                sum += (v as u32) + 1;
            }
        }

        sum
    }
}

type Action = Vec<u32>;

fn actions_for_state(
    state: &State,
) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];

    for i in 0..state.dice_value {
        let j = state.dice_value - i;

        if i >= j {
            break;
        }

        let i_is_open = state.tiles_open.get(i as usize).unwrap_or(&false);
        let j_is_open = state.tiles_open.get(j as usize).unwrap_or(&false);

        if *i_is_open && *j_is_open {
            actions.push(vec![i, j]);
        }
    }

    actions
}

pub fn shut_the_box() {
    let mut state_values: HashMap<State, u32> = HashMap::new();

    let initial_states = State::initial();

    let mut remaining_states: VecDeque<State> = VecDeque::from(initial_states);

    while let Some(working_state) = remaining_states.pop_front() {
        let actions = actions_for_state(&working_state);

        if actions.is_empty() {
            let terminal_value = working_state.score();
            state_values.insert(working_state, terminal_value);
        } else {
            for action in actions.iter() {
                
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {

    }
}