use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;

pub trait StateGraph<S, V> 
    where S: Copy {
    fn insert(&mut self, state: S);
    fn contains(&self, state: &S) -> bool;

    fn add_value_dependency(&mut self, state: &S, dependency: S);
    fn get_value_dependencies(&self, state: &S) -> Option<&Vec<S>>;

    fn add_dependent(&mut self, state: &S, dependent: S);
    fn get_dependents(&self, state: &S) -> Option<&Vec<S>>;

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
        s0: S,
    ) -> InMemoryStateGraph<S,V> {
        let mut graph: InMemoryStateGraph<S,V> = InMemoryStateGraph::new();

        let mut unvisited_states = VecDeque::from(vec![s0]);

        while let Some(working_state) = unvisited_states.pop_front() {
            println!("working_state: {:?}", working_state);
            let next_states = l(&working_state);
            println!("next_states: {:?}", next_states);

            if !graph.contains(&working_state) {
                graph.insert(working_state);   
            }

            if next_states.is_empty() {
                // terminal state reached
                println!("terminal state")
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

    fn add_value_dependency(&mut self, state: &S, dependency: S) {
        let maybe_state_node = self.states.get_mut(&state);

        match maybe_state_node {
            Some(state_node) => {
                println!("adding value dep {:?}, {:?}", state, dependency);
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
                2 => vec![2, 4, 1],
                3 => vec![2],
                4 => vec![],
                _ => vec![],
            }
        };

        let d_graph: InMemoryStateGraph<u32, f64> = InMemoryStateGraph::generate(l, s0);

        println!("{:?}", d_graph);
    }
}