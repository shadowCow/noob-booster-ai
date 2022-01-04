use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use crate::dyn_prog::state_dependency_graph::{
    InMemoryStateGraph,
    StateGraph,
};
use crate::dice_utils::{
    D6,
    TwoD6,
};

type Action = Vec<Tile>;

#[derive(Debug, PartialEq)]
pub enum Tile {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Tile {
    pub fn score(&self) -> u8 {
        match self {
            Tile::One => 1,
            Tile::Two => 2,
            Tile::Three => 3,
            Tile::Four => 4,
            Tile::Five => 5,
            Tile::Six => 6,
            Tile::Seven => 7,
            Tile::Eight => 8,
            Tile::Nine => 9,
        }
    }

    fn tile_combos_for_dice_roll(roll: u8) -> Vec<Action> {
        match roll {
            2 => vec![
                vec![Tile::Two],
            ],
            3 => vec![
                vec![Tile::Three],
                vec![Tile::One, Tile::Two],
            ],
            4 => vec![
                vec![Tile::Four],
                vec![Tile::One, Tile::Three],
            ],
            5 => vec![
                vec![Tile::Five],
                vec![Tile::One, Tile::Four],
                vec![Tile::Two, Tile::Three],
            ],
            6 => vec![
                vec![Tile::Six],
                vec![Tile::One, Tile::Five],
                vec![Tile::One, Tile::Two, Tile::Three],
                vec![Tile::Two, Tile::Four],
            ],
            7 => vec![
                vec![Tile::Seven],
                vec![Tile::One, Tile::Six],
                vec![Tile::One, Tile::Two, Tile::Four],
                vec![Tile::Two, Tile::Five],
                vec![Tile::Three, Tile::Four],
            ],
            8 => vec![
                vec![Tile::Eight],
                vec![Tile::One, Tile::Seven],
                vec![Tile::One, Tile::Two, Tile::Five],
                vec![Tile::One, Tile::Three, Tile::Four],
                vec![Tile::Two, Tile::Six],
                vec![Tile::Three, Tile::Five],
            ],
            9 => vec![
                vec![Tile::Nine],
                vec![Tile::One, Tile::Eight],
                vec![Tile::One, Tile::Two, Tile::Six],
                vec![Tile::One, Tile::Three, Tile::Five],
                vec![Tile::Two, Tile::Seven],
                vec![Tile::Two, Tile::Three, Tile::Four],
                vec![Tile::Three, Tile::Six],
                vec![Tile::Four, Tile::Five],
            ],
            10 => vec![
                vec![Tile::One, Tile::Nine],
                vec![Tile::One, Tile::Two, Tile::Seven],
                vec![Tile::One, Tile::Two, Tile::Three, Tile::Four],
                vec![Tile::One, Tile::Three, Tile::Six],
                vec![Tile::One, Tile::Four, Tile::Five],
                vec![Tile::Two, Tile::Eight],
                vec![Tile::Two, Tile::Three, Tile::Five],
                vec![Tile::Three, Tile::Seven],
                vec![Tile::Four, Tile::Six],
            ],
            11 => vec![
                vec![Tile::Two, Tile::Nine],
                vec![Tile::Two, Tile::One, Tile::Eight],
                vec![Tile::Two, Tile::One, Tile::Three, Tile::Five],
                vec![Tile::Two, Tile::Three, Tile::Six],
                vec![Tile::Two, Tile::Four, Tile::Five],
                vec![Tile::Three, Tile::Eight],
                vec![Tile::Three, Tile::One, Tile::Seven],
                vec![Tile::Four, Tile::Seven],
                vec![Tile::Four, Tile::One, Tile::Six],
                vec![Tile::Five, Tile::Six],
            ],
            12 => vec![
                vec![Tile::One, Tile::Two, Tile::Nine],
                vec![Tile::One, Tile::Two, Tile::Three, Tile::Six],
                vec![Tile::One, Tile::Two, Tile::Four, Tile::Five],
                vec![Tile::One, Tile::Three, Tile::Eight],
                vec![Tile::One, Tile::Four, Tile::Seven],
                vec![Tile::One, Tile::Five, Tile::Six],
                vec![Tile::Two, Tile::Three, Tile::Seven],
                vec![Tile::Two, Tile::Four, Tile::Six],

                vec![Tile::Three, Tile::Nine],
                vec![Tile::Three, Tile::Four, Tile::Five],

                vec![Tile::Four, Tile::Eight],
                vec![Tile::Five, Tile::Seven],
            ],
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct State {
    dice_value: u8,
    tiles_open: [bool; 9],
}

impl State {
    pub fn initial() -> Vec<State> {
        let mut states: Vec<State> = vec![];

        for i in 2..=12 {
            states.push(State::fresh(i));
        }
    
        states
    }

    pub fn fresh(dice_value: u8) -> State {
        State {
            dice_value,
            tiles_open: [true; 9],
        }
    }

    pub fn from(
        dice_value: u8,
        tiles_open: &[bool; 9],
    ) -> State {
        State {
            dice_value,
            tiles_open: *tiles_open,
        }
    }

    pub fn rolls_for(tiles_open: &[bool; 9]) -> Vec<State> {
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

    pub fn probability_of_roll(&self) -> f64 {
        TwoD6::probability(&(self.dice_value as u8))
    }

    pub fn reachable_next_states(&self) -> Vec<State> {
        let mut states: Vec<State> = vec![];

        for action in self.actions() {
            let states_after_action = self.possible_transitions(&action);

            for s_a in states_after_action {
                states.push(s_a);
            }
        }

        states
    }

    pub fn possible_transitions(&self, action: &Action) -> Vec<State> {
        let mut state_copy = *self;
        action.iter()
            .for_each(|x| {
                let index = x.score() - 1;
                state_copy.tiles_open[usize::from(index)] = false
            });

        State::rolls_for(&state_copy.tiles_open)
    }

    pub fn actions(&self) -> Vec<Action> {
        // println!("to {:?}", self.tiles_open);
        Tile::tile_combos_for_dice_roll(self.dice_value)
            .into_iter()
            .filter(|combo| {
                // println!("x {:?}", combo);
                combo.into_iter().all(|i| {
                    let index = i.score() - 1;
                    // println!("i, index {:?}, {:?}", i, index);
                    self.tiles_open[usize::from(index)] == true
                })
            })
            .collect()
    }

    pub fn best_action(
        &self,
        g: &dyn StateGraph<State,f64>,
    ) -> Option<(Action, f64)> {
        let mut best_action: Option<(Action, f64)> = None;
    
        for action in self.actions() {
            let action_value = compute_action_value(
                self,
                &action,
                g,
            );
    
            match action_value {
                Some(v) => {
                    let min_a = match best_action {
                        Some((_, v)) => v,
                        None => f64::MAX
                    };
    
                    if v < min_a {
                        best_action = Some((action, v));
                    }
                },
                None => {
                    best_action = None;
                    break;
                }
            }
        }
    
        best_action
    }
}


pub fn shut_the_box(initial_state: &State) -> Option<(Action, f64)> {
    let l = |s: &State| {
        s.reachable_next_states()
    };

    let mut d_graph: InMemoryStateGraph<State, f64> = InMemoryStateGraph::generate(
        l,
        initial_state,
    );
    // println!("graph {:?}", d_graph);
    println!("size {:?}", d_graph.count_states());

    let k = |s: &State, g: &dyn StateGraph<State,f64>| {
        if s.actions().is_empty() {
            Some(f64::from(s.score()))
        } else {
            s.best_action(g)
                .map(|(a, v)| v)
        }
    };

    d_graph.compute_values(k);

    initial_state.best_action(&d_graph)
}

fn compute_action_value(
    s: &State,
    a: &Action,
    g: &dyn StateGraph<State,f64>,
) -> Option<f64> {
    let next_states = s.possible_transitions(a);

    let mut value: Option<f64> = None;

    for d in next_states {
        match g.get_value(&d) {
            Some(v) => {
                let acc = value.unwrap_or(0.0);
                value = Some(acc + (v * d.probability_of_roll()))
            },
            None => {
                value = None;
                break;
            }
        }
    }

    value
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions() {
        let s0 = State {
            dice_value: 7,
            tiles_open: [
                false,
                true,
                true,
                true,
                true,
                false,
                true,
                true,
                true,
            ],
        };

        let actual_actions = s0.actions();
        let expected_actions: Vec<Action> = vec![
            vec![Tile::Seven],
            vec![Tile::Two, Tile::Five],
            vec![Tile::Three, Tile::Four],
        ];

        assert_eq!(actual_actions, expected_actions);
    }

    #[test]
    fn test_possible_transitions() {
        let s0 = State {
            dice_value: 7,
            tiles_open: [true; 9],
        };

        let action = vec![Tile::One, Tile::Six];

        let actual_transitions = s0.possible_transitions(&action);

        let expected_tiles_open = [
            false,
            true,
            true,
            true,
            true,
            false,
            true,
            true,
            true,
        ];
        let expected_transitions = State::rolls_for(&expected_tiles_open);

        assert_eq!(actual_transitions, expected_transitions);
    }

    #[test]
    fn test_reachable_next_states() {
        let s0 = State {
            dice_value: 3,
            tiles_open: [true; 9],
        };

        let mut next_tiles_1 = [true; 9];
        next_tiles_1[2] = false;

        let mut next_tiles_2 = [true; 9];
        next_tiles_2[0] = false;
        next_tiles_2[1] = false;

        let expected_reachable: Vec<State> = vec![
            next_tiles_1,
            next_tiles_2,
        ].iter()
            .map(|to| State::rolls_for(&to))
            .flatten()
            .collect();

        let actual_reachable = s0.reachable_next_states();

        assert_eq!(actual_reachable, expected_reachable);
    }

    #[test]
    fn test_score() {
        let s0 = State {
            dice_value: 10,
            tiles_open: [false; 9],
        };

        let actual_score = s0.score();
        let expected_score = 0;

        assert_eq!(actual_score, expected_score);
    }

    #[test]
    fn test_shut_the_box() {
        let s0 = State {
            dice_value: 4,
            tiles_open: [
                false,
                false,
                false,
                true,
                false,
                false,
                false,
                false,
                true,
            ],
        };

        let actual_best_action = shut_the_box(&s0);
        let expected_best_action = Some((vec![Tile::Four], 8.0));

        assert_eq!(actual_best_action, expected_best_action);
    }

    #[test]
    fn test_thing() {
        let s0 = State {
            dice_value: 12,
            tiles_open: [true; 9],
        };

        let best_action = shut_the_box(&s0);

        println!("best action {:?}", best_action);
    }
}