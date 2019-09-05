use std::fmt::Debug;

pub struct GenerativeEvaluationTree<T> where T: GenerativeEvaluationTreeNode<T> + Debug {
    root_node: T,
    tree_path: Vec<T>,
    max_depth: usize,
    search_completed: bool
}

impl <T> GenerativeEvaluationTree<T> where T: GenerativeEvaluationTreeNode<T> + Debug {
    pub fn new(
        root_node: T,
        max_depth: usize
    ) -> GenerativeEvaluationTree<T> {

        GenerativeEvaluationTree {
            root_node,
            tree_path: Vec::new(),
            max_depth,
            search_completed: false
        }
    }

    pub fn search(&mut self) -> Vec<&T> {
        while !self.search_completed {
            self.evaluate_next();
        }

        let mut full_path = vec![&self.root_node];
        for node in &self.tree_path {
            full_path.push(node);
        }
        full_path
    }

    fn evaluate_next(&mut self) {
        if self.tree_path.is_empty() {
            match self.root_node.request_next_child() {
                Some(child) => self.tree_path.push(child),
                None => self.search_completed = true
            }
        } else {
            let path_length = self.tree_path.len();
            let tail_node = self.tree_path.last_mut().unwrap();
            
            if path_length < self.max_depth {
                match tail_node.request_next_child() {
                    Some(child) => self.tree_path.push(child),
                    None => self.evaluate_and_prune()
                }
            } else {
                self.evaluate_and_prune();
            }
        }
    }

    fn evaluate_and_prune(&mut self) {
        let maybe_node_to_prune = self.tree_path.pop();
        if let Some(mut node_to_prune) = maybe_node_to_prune {
            let abort_early = node_to_prune.evaluate();
            if abort_early {
                self.search_completed = true;
            } else {
                if self.tree_path.is_empty() {
                    self.root_node.on_child_pruned(node_to_prune);
                } else {
                    self.tree_path.last_mut().unwrap().on_child_pruned(node_to_prune);
                }
            }
        }
    }
}

pub trait GenerativeEvaluationTreeNode<T> {
    fn on_child_pruned(&mut self, child: T);
    fn request_next_child(&self) -> Option<T>;
    fn evaluate(&mut self) -> bool;
}

mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
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


    impl GenerativeEvaluationTreeNode<DummyNode> for DummyNode {
        fn on_child_pruned(&mut self, child: DummyNode) {
            self.next_child += 1;
            if child.max_child_value > self.max_child_value {
                self.max_child_value = child.max_child_value;
            }
        }

        fn request_next_child(&self) -> Option<DummyNode> {
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

        fn evaluate(&mut self) -> bool {
            if self.next_child == 0 {
                self.max_child_value = self.id;
            }
            false
        }
    }

    #[test]
    fn test_new_tree() {
        let new_tree = GenerativeEvaluationTree::new(
            DummyNode::new(1),
            2
        );

        assert_eq!(true, new_tree.tree_path.is_empty());
        assert_eq!(2, new_tree.max_depth);
        assert_eq!(DummyNode::new(1), new_tree.root_node);
        assert_eq!(false, new_tree.search_completed);
    }

    #[test]
    fn test_my_tree_search() {
        let mut new_tree = GenerativeEvaluationTree::new(
            DummyNode::new(1),
            2
        );

        let search_results = new_tree.search();
        let expected_results = vec![
            &DummyNode {
                id: 1,
                next_child: 2,
                max_child_value: 5
            }
        ];

        assert_eq!(expected_results, search_results);
    }
}