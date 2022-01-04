use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;
use std::cmp::max;
use std::iter::FromIterator;

pub trait StateGraph<S, V> 
    where S: Copy {
    fn insert(&mut self, state: S);
    fn contains(&self, state: &S) -> bool;
    fn count_states(&self) -> usize;

    fn add_value_dependency(&mut self, state: &S, dependency: S);
    fn get_value_dependencies(&self, state: &S) -> Option<&Vec<S>>;

    fn add_dependent(&mut self, state: &S, dependent: S);
    fn get_dependents(&self, state: &S) -> Option<&Vec<S>>;

    fn get_terminal_states(&self) -> Vec<S>;

    fn set_value(&mut self, state: &S, value: V);
    fn get_value(&self, state: &S) -> Option<&V>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct InMemoryStateGraph<S, V>
    where S: Hash + Eq + Debug + Copy,
          V: PartialEq + Debug {

    states: HashMap<S, StateNode<S,V>>,
}

impl <S,V> InMemoryStateGraph<S,V>
    where S: Hash + Eq + Debug + Copy,
          V: PartialEq + Debug {
    
    pub fn new() -> InMemoryStateGraph<S,V> {
        InMemoryStateGraph {
            states: HashMap::new(),
        }
    }

    pub fn generate(
        l: fn(&S) -> Vec<S>,
        initial_states: Vec<S>,
    ) -> InMemoryStateGraph<S,V> {
        let mut graph: InMemoryStateGraph<S,V> = InMemoryStateGraph::new();

        let mut unvisited_states: VecDeque<S> = VecDeque::from(initial_states);

        while let Some(working_state) = unvisited_states.pop_front() {
            // println!("working_state: {:?}", working_state);
            let next_states = l(&working_state);
            // println!("next_states: {:?}", next_states);

            if !graph.contains(&working_state) {
                graph.insert(working_state);   
            }

            if next_states.is_empty() {
                // terminal state reached
                // println!("terminal state")
            } else {
                for n_s in next_states.iter() {
                    graph.add_value_dependency(&working_state, *n_s);
    
                    if !graph.contains(n_s) {
                        graph.insert(*n_s);
                        
                        unvisited_states.push_front(*n_s);
                    }

                    graph.add_dependent(n_s, working_state);
                }
            }
        }

        graph
    }

    pub fn compute_values(
        &mut self,
        k: fn(&S, &dyn StateGraph<S,V>) -> Option<V>,
    ) {
        let mut states_to_evaluate = VecDeque::from(self.get_terminal_states());
        // println!("states_to_evaluate {:?}", states_to_evaluate);
        while let Some(working_state) = states_to_evaluate.pop_front() {
            match k(&working_state, self) {
                Some(state_value) => {
                    self.set_value(&working_state, state_value);
                    match self.get_dependents(&working_state) {
                        Some(ds) => {
                            ds.iter()
                              .filter(|x| self.get_value(x).is_none())
                              .for_each(|x| states_to_evaluate.push_back(*x))
                        },
                        None => {}
                    }
                },
                None => {
                    states_to_evaluate.push_back(working_state);
                }
            }
        }
    }
        

}

#[derive(Debug, PartialEq, Clone)]
struct StateNode<S,V>
    where S: Hash + Eq + Debug,
          V: PartialEq + Debug {
    value_dependencies: Vec<S>,
    dependents: Vec<S>,
    value: Option<V>,
}

impl <S,V> StateNode<S,V>
    where S: Hash + Eq + Debug,
          V: PartialEq + Debug {

    pub fn new() -> StateNode<S,V> {
        StateNode {
            value_dependencies: vec![],
            dependents: vec![],
            value: None,
        }
    }
}

impl <S, V> StateGraph<S, V> for InMemoryStateGraph<S, V>
    where S: Eq + Hash + Debug + Copy,
          V: PartialEq + Debug {

    fn insert(&mut self, state: S) {
        self.states.insert(state, StateNode::new());
    }

    fn contains(&self, state: &S) -> bool {
        self.states.contains_key(state)
    }

    fn count_states(&self) -> usize {
        self.states.len()
    }

    fn add_value_dependency(&mut self, state: &S, dependency: S) {
        let maybe_state_node = self.states.get_mut(&state);

        match maybe_state_node {
            Some(state_node) => {
                // println!("adding value dep {:?}, {:?}", state, dependency);
                state_node.value_dependencies.push(dependency);
            },
            None => {
                // nothing?  create?
            },
        }
        
    }
    fn get_value_dependencies(&self, state: &S) -> Option<&Vec<S>> {
        self.states
            .get(&state)
            .map(|x| &x.value_dependencies)
    }

    fn add_dependent(&mut self, state: &S, dependent: S) {
        let maybe_state_node = self.states.get_mut(&state);

        match maybe_state_node {
            Some(state_node) => {
                state_node.dependents.push(dependent);
            },
            None => {
                // nothing?  create?
            },
        }
    }

    fn get_dependents(&self, state: &S) -> Option<&Vec<S>> {
        self.states
            .get(&state)
            .map(|x| &x.dependents)
    }

    fn get_terminal_states(&self) -> Vec<S> {
        self.states
            .iter()
            .filter_map(|(k,v)| {
                if v.value_dependencies.is_empty() {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect()
    }

    fn set_value(&mut self, state: &S, value: V) {
        let maybe_state_node = self.states.get_mut(&state);

        match maybe_state_node {
            Some(state_node) => {
                state_node.value = Some(value);
            },
            None => {
                // nothing?  create?
            },
        }
    }
    fn get_value(&self, state: &S) -> Option<&V> {
        self.states
            .get(&state)
            .map(|x| x.value.as_ref())
            .flatten()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_InMemoryStateGraph() {
        let s0: u32 = 1;
        let l = |s: &u32| {
            match s {
                1 => vec![2, 3],
                2 => vec![4],
                3 => vec![5],
                4 => vec![],
                5 => vec![],
                _ => vec![],
            }
        };

        let mut d_graph: InMemoryStateGraph<u32, f64> = InMemoryStateGraph::generate(l, vec![s0]);

        println!("{:?}", d_graph);

        let k = |s: &u32, g: &dyn StateGraph<u32, f64>| {
            match s {
                1 => {
                    g.get_value(&2)
                     .zip(g.get_value(&3))
                     .map(|(a,b)| a.max(*b))
                },
                2 => g.get_value(&4).map(|x| *x),
                3 => g.get_value(&5).map(|x| *x),
                4 => Some(1.0),
                5 => Some(2.0),
                _ => None
            }
        };

        d_graph.compute_values(k);

        println!("{:?}", d_graph);
    }
}