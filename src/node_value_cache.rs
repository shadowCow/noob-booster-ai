use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub trait NodeValueCache<T,V> where V: Debug {
    fn save_value(&mut self, node: &T, value: V);
    fn get_value(&self, node: &T) -> Option<&V>;
    fn size(&self) -> usize;
}

#[derive(Debug)]
pub struct NoOpCache {}

impl <T,V> NodeValueCache<T,V> for NoOpCache where V: Debug {
    fn save_value(&mut self, node: &T, value: V) {}
    fn get_value(&self, node: &T) -> Option<&V> { None }
    fn size(&self) -> usize { 0 }
}

pub struct InMemoryNodeValueCache<T,K,V>
    where K: Eq + Hash + Debug,
          V: Debug {
    value_map: HashMap<K,V>,
    get_key_for_node: fn(&T) -> K
}

impl <T,K,V> Debug for InMemoryNodeValueCache<T,K,V>
    where K: Eq + Hash + Debug,
          V: Debug {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value_map.fmt(formatter)
    }
}

impl <T,K,V> InMemoryNodeValueCache<T,K,V>
    where K: Eq + Hash + Debug,
          V: Debug {
    pub fn new(get_key_for_node: fn(&T) -> K) -> InMemoryNodeValueCache<T,K,V> {
        InMemoryNodeValueCache {
            value_map: HashMap::new(),
            get_key_for_node
        }
    }
}

impl <T,K,V> NodeValueCache<T,V> for InMemoryNodeValueCache<T,K,V> 
    where K: Eq + Hash + Debug, 
          V: Debug {

    fn save_value(&mut self, node: &T, value: V) {
        let key = (self.get_key_for_node)(node);
        self.value_map.entry(key).or_insert(value);
    }

    fn get_value(&self, node: &T) -> Option<&V> {
        let key = (self.get_key_for_node)(node);
        self.value_map.get(&key)
    }

    fn size(&self) -> usize {
        self.value_map.len()
    }
}


mod tests {
    use super::*;

    struct DummyNode {
        id: u32,
    }

    fn dummy_get_key_for_node(node: &DummyNode) -> u32 {
        node.id
    }

    #[test]
    fn test_in_memory_node_value_service() {

        let mut service = InMemoryNodeValueCache::<DummyNode, u32, f64>::new(dummy_get_key_for_node);

        let (node,v) = (DummyNode { id: 1 }, 0.6);
        
        let expected_value = None;
        let actual_value = service.get_value(&node);
        assert_eq!(expected_value, actual_value);

        service.save_value(&node, v);

        let expected_value_after_save = Some(&0.6);
        let actual_value_after_save = service.get_value(&node);
        assert_eq!(expected_value_after_save, actual_value_after_save);
    }
}