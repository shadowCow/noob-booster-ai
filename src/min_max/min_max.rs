use std::fmt::Debug;

pub trait StateValueEstimator<State>
    where State: Debug + Clone + PartialEq {

    fn estimate_win_probability(state: State) -> f64;

}

pub struct MinMaxTree {

}

impl MinMaxTree {
    
}