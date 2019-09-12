use crate::generative_evaluation_tree::GenerativeEvaluationTreeNode;
use crate::generative_evaluation_tree::GenerativeEvaluationTree;
use crate::outcomes::BinaryOutcome;

#[derive(Clone, Debug)]
struct TrianglePegState {
    board: [bool; 15]
}

impl TrianglePegState {
    fn new(empty_hole: usize) -> TrianglePegState {
        let mut board = [true; 15];
        board[empty_hole] = false;

        TrianglePegState { board }
    }

    fn count_pegs(&self) -> u32 {
        let mut count = 0;
        for b in self.board.iter() {
            if *b { count += 1; }
        }
        count
    }

    fn get_legal_moves(&self) -> Vec<TrianglePegAction> {
        let mut legal_moves = Vec::new();
        for (i, b) in self.board.iter().enumerate() {
            if *b {
                match i {
                    0 => {
                        if self.board[1] && !self.board[3] {
                            legal_moves.push(TrianglePegAction{from:0,over:1,to:3});
                        }
                        if self.board[2] && !self.board[5] {
                            legal_moves.push(TrianglePegAction{from:0,over:2,to:5});
                        }
                    }
                    1 => {
                        if self.board[3] && !self.board[6] {
                            legal_moves.push(TrianglePegAction{from:1,over:3,to:6});
                        }
                        if self.board[4] && !self.board[8] {
                            legal_moves.push(TrianglePegAction{from:1,over:4,to:8});
                        }
                    }
                    2 => {
                        if self.board[4] && !self.board[7] {
                            legal_moves.push(TrianglePegAction{from:2,over:4,to:7});
                        }
                        if self.board[5] && !self.board[9] {
                            legal_moves.push(TrianglePegAction{from:2,over:5,to:9});
                        }
                    }
                    3 => {
                        if self.board[1] && !self.board[0] {
                            legal_moves.push(TrianglePegAction{from:3,over:1,to:0});
                        }
                        if self.board[4] && !self.board[5] {
                            legal_moves.push(TrianglePegAction{from:3,over:4,to:5});
                        }
                        if self.board[6] && !self.board[10] {
                            legal_moves.push(TrianglePegAction{from:3,over:6,to:10});
                        }
                        if self.board[7] && !self.board[12] {
                            legal_moves.push(TrianglePegAction{from:3,over:7,to:12});
                        }
                    }
                    4 => {
                        if self.board[7] && !self.board[11] {
                            legal_moves.push(TrianglePegAction{from:4,over:7,to:11});
                        }
                        if self.board[8] && !self.board[13] {
                            legal_moves.push(TrianglePegAction{from:4,over:8,to:13});
                        }
                    }
                    5 => {
                        if self.board[2] && !self.board[0] {
                            legal_moves.push(TrianglePegAction{from:5,over:2,to:0});
                        }
                        if self.board[4] && !self.board[3] {
                            legal_moves.push(TrianglePegAction{from:5,over:4,to:3});
                        }
                        if self.board[8] && !self.board[12] {
                            legal_moves.push(TrianglePegAction{from:5,over:8,to:12});
                        }
                        if self.board[9] && !self.board[14] {
                            legal_moves.push(TrianglePegAction{from:5,over:9,to:14});
                        }
                    }
                    6 => {
                        if self.board[3] && !self.board[1] {
                            legal_moves.push(TrianglePegAction{from:6,over:3,to:1});
                        }
                        if self.board[7] && !self.board[8] {
                            legal_moves.push(TrianglePegAction{from:6,over:7,to:8});
                        }
                    }
                    7 => {
                        if self.board[4] && !self.board[2] {
                            legal_moves.push(TrianglePegAction{from:7,over:4,to:2});
                        }
                        if self.board[8] && !self.board[9] {
                            legal_moves.push(TrianglePegAction{from:7,over:8,to:9});
                        }
                    }
                    8 => {
                        if self.board[4] && !self.board[1] {
                            legal_moves.push(TrianglePegAction{from:8,over:4,to:1});
                        }
                        if self.board[7] && !self.board[6] {
                            legal_moves.push(TrianglePegAction{from:8,over:7,to:6});
                        }
                    }
                    9 => {
                        if self.board[5] && !self.board[2] {
                            legal_moves.push(TrianglePegAction{from:9,over:5,to:2});
                        }
                        if self.board[8] && !self.board[7] {
                            legal_moves.push(TrianglePegAction{from:9,over:8,to:7});
                        }
                    }
                    10 => {
                        if self.board[6] && !self.board[3] {
                            legal_moves.push(TrianglePegAction{from:10,over:6,to:3});
                        }
                        if self.board[11] && !self.board[12] {
                            legal_moves.push(TrianglePegAction{from:10,over:11,to:12});
                        }
                    }
                    11 => {
                        if self.board[7] && !self.board[4] {
                            legal_moves.push(TrianglePegAction{from:11,over:7,to:4});
                        }
                        if self.board[12] && !self.board[13] {
                            legal_moves.push(TrianglePegAction{from:11,over:12,to:13});
                        }
                    }
                    12 => {
                        if self.board[11] && !self.board[10] {
                            legal_moves.push(TrianglePegAction{from:12,over:11,to:10});
                        }
                        if self.board[7] && !self.board[3] {
                            legal_moves.push(TrianglePegAction{from:12,over:7,to:3});
                        }
                        if self.board[8] && !self.board[5] {
                            legal_moves.push(TrianglePegAction{from:12,over:8,to:5});
                        }
                        if self.board[13] && !self.board[14] {
                            legal_moves.push(TrianglePegAction{from:12,over:13,to:14});
                        }
                    }
                    13 => {
                        if self.board[8] && !self.board[4] {
                            legal_moves.push(TrianglePegAction{from:13,over:8,to:4});
                        }
                        if self.board[12] && !self.board[11] {
                            legal_moves.push(TrianglePegAction{from:13,over:12,to:11});
                        }
                    }
                    14 => {
                        if self.board[9] && !self.board[5] {
                            legal_moves.push(TrianglePegAction{from:14,over:9,to:5});
                        }
                        if self.board[13] && !self.board[12] {
                            legal_moves.push(TrianglePegAction{from:14,over:13,to:12});
                        }
                    }
                    _ => panic!("invalid index for board")
                }
            }
        }

        legal_moves
    }

    fn board_after_move(&self, action: TrianglePegAction) -> TrianglePegState {
        let mut board_copy = [false; 15];
        board_copy.copy_from_slice(&self.board);

        board_copy[action.from] = false;
        board_copy[action.over] = false;
        board_copy[action.to] = true;

        TrianglePegState { board: board_copy }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TrianglePegAction {
    from: usize,
    over: usize,
    to: usize
}

#[derive(Debug)]
struct TrianglePegNode {
    state: TrianglePegState,
    legal_moves: Vec<TrianglePegAction>,
    move_index: usize
}


impl GenerativeEvaluationTreeNode<TrianglePegNode, BinaryOutcome> for TrianglePegNode {
    fn on_child_pruned(&mut self, child: TrianglePegNode) {
        self.move_index += 1;
    }

    fn request_next_child(&self) -> Option<TrianglePegNode> {
        if self.move_index < self.legal_moves.len() {
            let current_move = self.legal_moves[self.move_index];
            let next_state = self.state.board_after_move(current_move);
            let next_legal_moves = next_state.get_legal_moves();

            Some(TrianglePegNode {
                state: next_state,
                legal_moves: next_legal_moves,
                move_index: 0
            })
        } else {
            None
        }
    }

    fn on_children_completed(&mut self) -> BinaryOutcome {
        if self.state.count_pegs() == 1 {
            BinaryOutcome::Win
        } else {
            BinaryOutcome::Lose
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_triangle_peg_search() {
        let state = TrianglePegState::new(12);
        let legal_moves = state.get_legal_moves();

        let root_node = TrianglePegNode {
            state,
            legal_moves,
            move_index: 0
        };

        let mut tree = GenerativeEvaluationTree::<TrianglePegNode, BinaryOutcome>::new(
            root_node,
            100,
            Some(BinaryOutcome::Win)
        );

        let search_results = tree.search();
        let action_list: Vec<&TrianglePegAction> = search_results.iter().map(|x| {
            &x.legal_moves[x.move_index]
        }).collect();
        
        let expected_action_list = vec![
            &TrianglePegAction { from: 3, over: 7, to: 12 },
            &TrianglePegAction { from: 0, over: 1, to: 3 },
            &TrianglePegAction { from: 2, over: 4, to: 7 },
            &TrianglePegAction { from: 6, over: 3, to: 1 },
            &TrianglePegAction { from: 9, over: 5, to: 2 },
            &TrianglePegAction { from: 11, over: 7, to: 4 },
            &TrianglePegAction { from: 12, over: 8, to: 5 },
            &TrianglePegAction { from: 1, over: 4, to: 8 },
            &TrianglePegAction { from: 2, over: 5, to: 9 },
            &TrianglePegAction { from: 14, over: 9, to: 5 },
            &TrianglePegAction { from: 5, over: 8, to: 12 },
            &TrianglePegAction { from: 13, over: 12, to: 11 },
            &TrianglePegAction { from: 10, over: 11, to: 12 }
        ];

        assert_eq!(expected_action_list, action_list);
    }
}