use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub trait StateValueCache<S> {
    fn put(&mut self, state: &S, value: f64);
    fn get(&self, state: &S) -> Option<&f64>;
    fn size(&self) -> usize;
}


#[derive(Debug)]
pub struct NoOpCache {}

impl <S> StateValueCache<S> for NoOpCache {
    fn put(&mut self, state: &S, value: f64) {}
    fn get(&self, state: &S) -> Option<&f64> { None }
    fn size(&self) -> usize { 0 }
}

pub struct InMemoryStateValueCache<S,K>
    where K: Eq + Hash + Debug {
    value_map: HashMap<K,f64>,
    get_key_for_state: fn(&S) -> K
}

impl <S,K> Debug for InMemoryStateValueCache<S,K>
    where K: Eq + Hash + Debug {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value_map.fmt(formatter)
    }
}

impl <S,K> InMemoryStateValueCache<S,K>
    where K: Eq + Hash + Debug {
    pub fn new(get_key_for_state: fn(&S) -> K) -> InMemoryStateValueCache<S,K> {
        InMemoryStateValueCache {
            value_map: HashMap::new(),
            get_key_for_state
        }
    }
}

impl <S,K> StateValueCache<S> for InMemoryStateValueCache<S,K> 
    where K: Eq + Hash + Debug {

    fn put(&mut self, state: &S, value: f64) {
        let key = (self.get_key_for_state)(state);
        self.value_map.entry(key).or_insert(value);
    }

    fn get(&self, state: &S) -> Option<&f64> {
        let key = (self.get_key_for_state)(state);
        self.value_map.get(&key)
    }

    fn size(&self) -> usize {
        self.value_map.len()
    }
}


mod tests {
    use super::*;

    struct DummyState {
        id: u32,
    }

    fn dummy_get_key_for_state(node: &DummyState) -> u32 {
        node.id
    }

    #[test]
    fn test_in_memory_state_value_cache() {

        let mut service = InMemoryStateValueCache::<DummyState, u32>::new(
            dummy_get_key_for_state
        );

        let (state,v) = (DummyState { id: 1 }, 0.6);
        
        let expected_value = None;
        let actual_value = service.get(&state);
        assert_eq!(expected_value, actual_value);

        service.put(&state, v);

        let expected_value_after_save = Some(&0.6);
        let actual_value_after_save = service.get(&state);
        assert_eq!(expected_value_after_save, actual_value_after_save);
    }
}