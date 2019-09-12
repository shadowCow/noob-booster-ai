use std::collections::HashMap;
use std::hash::Hash;

pub trait NodeValueService<T,V> {
    fn save_value(&mut self, node: &T, value: V);
    fn get_value(&self, node: &T) -> Option<&V>;
}

pub struct InMemoryNodeValueService<T,K,V> where K: Eq + Hash {
    value_map: HashMap<K,V>,
    get_key_for_node: fn(&T) -> K
}

impl <T,K,V> InMemoryNodeValueService<T,K,V> where K: Eq + Hash {
    fn new(get_key_for_node: fn(&T) -> K) -> InMemoryNodeValueService<T,K,V> {
        InMemoryNodeValueService {
            value_map: HashMap::new(),
            get_key_for_node
        }
    }
}

impl <T,K,V> NodeValueService<T,V> for InMemoryNodeValueService<T,K,V> 
    where K: Eq + Hash {

    fn save_value(&mut self, node: &T, value: V) {
        let key = (self.get_key_for_node)(node);
        self.value_map.entry(key).or_insert(value);
    }

    fn get_value(&self, node: &T) -> Option<&V> {
        let key = (self.get_key_for_node)(node);
        self.value_map.get(&key)
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

        let mut service = InMemoryNodeValueService::<DummyNode, u32, f64>::new(dummy_get_key_for_node);

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