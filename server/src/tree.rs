use std::fmt::Debug;

use crate::node_value_cache::NodeValueCache;
use crate::node_value_cache::NoOpCache;

pub trait TreeNode<T,V>
    where T: TreeNode<T,V> + Debug + Clone + PartialEq,
          V: PartialEq + Debug + Clone {

    fn request_next_child(&mut self) -> Option<T>;
    fn on_child_pruned(&mut self, child: T, child_value: V);
    fn on_all_children_pruned(&mut self) -> V;
}

pub struct TreeEvaluator<T,V,C>
    where T: TreeNode<T,V> + Debug + Clone + PartialEq,
          V: PartialEq + Debug + Clone,
          C: NodeValueCache<T,V> + Debug {

    tree_path: Vec<T>,
    node_value_cache: C,
    max_depth: usize,
    maybe_early_stopping_value: Option<V>,
    is_finished: bool
}

impl <T,V,C> TreeEvaluator<T,V,C>
    where T: TreeNode<T,V> + Debug + Clone + PartialEq,
          V: PartialEq + Debug + Clone,
          C: NodeValueCache<T,V> + Debug {

    pub fn new(
        root_node: T,
        node_value_cache: C,
        max_depth: usize,
        maybe_early_stopping_value: Option<V>
    ) -> TreeEvaluator<T,V,C> {
        TreeEvaluator {
            tree_path: vec![root_node],
            node_value_cache,
            max_depth,
            maybe_early_stopping_value,
            is_finished: false
        }
    }

    pub fn root_node(&self) -> &T {
        &self.tree_path[0]
    }

    pub fn get_tree_path(&self) -> Vec<&T> {
        let mut path_copy: Vec<&T> = Vec::new();
        for node in &self.tree_path {
            path_copy.push(node);
        }
        path_copy
    }

    pub fn print_cache(&self) {
        println!("cache size {:?}", self.node_value_cache.size());
        println!("node value cache {:?}", self.node_value_cache);
    }

    fn print_path(&self, node_fmt: fn(&T) -> String) {
        println!("");
        for node in &self.tree_path {
            print!("node {:?} ,", node_fmt(node));
        }
        println!("");
    }

    pub fn search(&mut self) {
        while !self.is_finished {
            self.next();
        }
    }

    pub fn search_with_max_visits(&mut self, max_node_visits: u32, node_fmt: fn(&T) -> String) {
        let mut nodes_visited = 0;
        let mut last_tree_path = self.tree_path.clone();
        while !self.is_finished && nodes_visited < max_node_visits {
            self.next();
            nodes_visited += 1;
            // println!("visit {:?}", nodes_visited);
            
            if last_tree_path == self.tree_path {
                println!("samesies after visit {:?}", nodes_visited);
                self.print_path(node_fmt);
                break;
            }
            if nodes_visited % 100000 == 0 {
                println!("another 100k {:?}", nodes_visited);
                
                self.print_path(node_fmt);
                // self.print_cache();
            }
            last_tree_path = self.tree_path.clone();
        }
    }

    fn next(&mut self) {
        let path_length = self.tree_path.len();
        let mut tail_node = self.tree_path.last_mut().unwrap();
        
        if path_length <= self.max_depth {
            match tail_node.request_next_child() {
                Some(child) => {
                    let maybe_already_evaluated = self.node_value_cache.get_value(&tail_node);
                    match maybe_already_evaluated {
                        Some(value) => {
                            // println!("already evaluated, not branching");
                            tail_node.on_child_pruned(child, value.clone());
                        }
                        None => {
                            // println!("not evaluated, branching");
                            self.tree_path.push(child)
                        }
                    }
                }
                None => {
                    // println!("no more children, pruning");
                    self.prune();
                }
            }    
        } else {
            // println!("max depth, pruning");
            self.prune();
        }
    }

    fn prune(&mut self) {
        let path_length = self.tree_path.len();

        if path_length == 1 {
            self.is_finished = true;
        } else {
            let mut node_to_prune = self.tree_path.pop().unwrap();

            let node_value = node_to_prune.on_all_children_pruned();
            if self.should_stop_early(&node_value) {
                self.is_finished = true;
            } else {
                if let None = self.node_value_cache.get_value(&node_to_prune) {
                    self.node_value_cache.save_value(&node_to_prune, node_value.clone());
                }
                
                self.tree_path.last_mut().unwrap().on_child_pruned(node_to_prune, node_value);
            }
        }
    }

    fn should_stop_early(&self, node_value: &V) -> bool {
        match &self.maybe_early_stopping_value {
            Some(early_stopping_value) => *early_stopping_value == *node_value,
            None => false
        }
    }

}

mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct DummyNode {
        id: u32,
        next_child: u32,
        max_child_value: u32,
    }
    impl DummyNode {
        fn new(id: u32) -> DummyNode {
            DummyNode {
                id,
                next_child: 0,
                max_child_value: 0
            }            
        }
    }


    impl TreeNode<DummyNode,u32> for DummyNode {
        fn on_child_pruned(&mut self, child: DummyNode, child_value: u32) {
            self.next_child += 1;
            if child.max_child_value > self.max_child_value {
                self.max_child_value = child.max_child_value;
            }
        }

        fn request_next_child(&mut self) -> Option<DummyNode> {
            if self.next_child < 2 {
                match (self.id, self.next_child) {
                    (1, 0) => Some(DummyNode::new(2)),
                    (1, 1) => Some(DummyNode::new(3)),
                    (2, 0) => None,
                    (3, 0) => Some(DummyNode::new(4)),
                    (3, 1) => Some(DummyNode::new(5)),
                    _ => None
                }
            } else {
                None
            }
        }

        fn on_all_children_pruned(&mut self) -> u32 {
            if self.next_child == 0 {
                self.max_child_value = self.id;
            }
            self.max_child_value
        }
    }

    #[test]
    fn test_new_tree() {
        let new_tree = TreeEvaluator::new(
            DummyNode::new(1),
            NoOpCache {},
            2,
            None
        );

        assert_eq!(1, new_tree.tree_path.len());
        assert_eq!(2, new_tree.max_depth);
        assert_eq!(&DummyNode::new(1), new_tree.root_node());
        assert_eq!(false, new_tree.is_finished);
    }

    #[test]
    fn test_my_tree_search() {
        let mut new_tree = TreeEvaluator::new(
            DummyNode::new(1),
            NoOpCache {},
            2,
            None
        );

        new_tree.search();
        let search_results = new_tree.tree_path;
        let expected_results = vec![
            DummyNode {
                id: 1,
                next_child: 2,
                max_child_value: 5
            }
        ];

        assert_eq!(expected_results, search_results);
    }
}