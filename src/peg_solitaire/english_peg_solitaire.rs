use crate::generative_evaluation_tree::GenerativeEvaluationTreeNode;
use crate::generative_evaluation_tree::GenerativeEvaluationTree;
use crate::outcomes::BinaryOutcome;

type Board = [bool; 33];
struct EnglishPegState {
    board: Board
}

impl EnglishPegState {
    fn new() -> EnglishPegState {
        EnglishPegState {
            board: make_starting_board()
        }
    }
}

impl std::fmt::Debug for EnglishPegState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.board[..].fmt(formatter)
    }
}

fn make_starting_board() -> Board {
    let mut board: Board = [true; 33];
    board[16] = false;
    board
}

fn count_pegs(board: &Board) -> u32 {
    let mut count = 0;
    for p in board.iter() {
        if *p { count += 1; }
    }
    count
}

fn get_legal_moves(board: &Board) -> Vec<EnglishPegMove> {
    let mut legal_moves = Vec::new();

    for (i,p) in board.iter().enumerate() {
        if *p {
            if let Some(left) = get_left_move(board, i) {
                legal_moves.push(left);
            }
            if let Some(right) = get_right_move(board, i) {
                legal_moves.push(right);
            }
            if let Some(up) = get_up_move(board, i) {
                legal_moves.push(up);
            }
            if let Some(down) = get_down_move(board, i) {
                legal_moves.push(down);
            }
        }
    }

    legal_moves
}

fn get_left_move(board: &Board, index: usize) -> Option<EnglishPegMove> {
    
    let has_two_on_left = match index {
        2|5|29|32 => true,
        x if x >= 8 && x <= 12 => true,
        x if x >= 15 && x <= 19 => true,
        x if x >= 22 && x <= 26 => true,
        _ => false
    };

    if has_two_on_left && board[index - 1] && !board[index - 2] {
        Some(EnglishPegMove {from: index, over: index - 1, to: index - 2})
    } else {
        None
    }
}

fn get_right_move(board: &Board, index: usize) -> Option<EnglishPegMove> {
    let has_two_on_right = match index {
        0|3|27|30 => true,
        x if x >= 6 && x <= 10 => true,
        x if x >= 13 && x <= 17 => true,
        x if x >= 20 && x <= 24 => true,
        _ => false
    };

    if has_two_on_right && board[index + 1] && !board[index + 2] {
        Some(EnglishPegMove {from: index, over: index + 1, to: index + 2})
    } else {
        None
    }
}

fn get_up_move(board: &Board, index: usize) -> Option<EnglishPegMove> {
    match index {
        x if x >= 8 && x <= 10 => {
            if board[index - 5] && !board[index - 8] {
                Some(EnglishPegMove {from: index, over: index - 5, to: index - 8})
            } else {
                None
            }
        },
        x if x >= 15 && x <= 17 => {
            if board[index - 7] && !board[index - 12] {
                Some(EnglishPegMove {from: index, over: index - 7, to: index - 12})
            } else {
                None
            }
        },
        x if x >= 20 && x <= 26 => {
            if board[index - 7] && !board[index - 14] {
                Some(EnglishPegMove {from: index, over: index - 7, to: index - 14})
            } else {
                None
            }
        },
        x if x >= 27 && x <= 29 => {
            if board[index - 5] && !board[index - 12] {
                Some(EnglishPegMove {from: index, over: index - 5, to: index - 12})
            } else {
                None
            }
        },
        x if x >= 30 && x <= 32 => {
            if board[index - 3] && !board[index - 8] {
                Some(EnglishPegMove {from: index, over: index - 3, to: index - 8})
            } else {
                None
            }
        }
        _ => None
    }
}


fn get_down_move(board: &Board, index: usize) -> Option<EnglishPegMove> {
    if index >= 0 && index <= 2 {
        if board[index + 3] && !board[index + 5] {
            Some(EnglishPegMove {from: index, over: index + 3, to: index + 5})
        } else {
            None
        }
    } else if index >= 3 && index <= 5 {
        if board[index + 5] && !board[index + 12] {
            Some(EnglishPegMove {from: index, over: index + 5, to: index + 12})
        } else {
            None
        }
    } else if index >= 6 && index <= 12 {
        if board[index + 7] && !board[index + 14] {
            Some(EnglishPegMove {from: index, over: index + 7, to: index + 14})
        } else {
            None
        }
    } else if index >= 15 && index <= 17 {
        if board[index + 7] && !board[index + 12] {
            Some(EnglishPegMove {from: index, over: index + 7, to: index + 12})
        } else {
            None
        }
    } else if index >= 22 && index <= 24 {
        if board[index + 5] && !board[index + 8] {
            Some(EnglishPegMove {from: index, over: index + 5, to: index + 8})
        } else {
            None
        }
    } else {
        None
    }
}

fn board_after_move(board: &Board, action: EnglishPegMove) -> Board {
    let mut board_copy = [false; 33];
    board_copy.copy_from_slice(board);

    if !board_copy[action.from] || !board_copy[action.over] || board_copy[action.to] {
        panic!("oops - this move isn't valid");
    }

    board_copy[action.from] = false;
    board_copy[action.over] = false;
    board_copy[action.to] = true;

    board_copy
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EnglishPegMove {
    from: usize,
    over: usize,
    to: usize
}

#[derive(Debug)]
struct EnglishPegNode {
    state: EnglishPegState,
    legal_moves: Vec<EnglishPegMove>,
    move_index: usize
}

enum Outcome {
    Win,
    Lose
}

impl GenerativeEvaluationTreeNode<EnglishPegNode, BinaryOutcome> for EnglishPegNode {
    fn on_child_pruned(&mut self, child: EnglishPegNode) {
        self.move_index += 1;
    }

    fn request_next_child(&self) -> Option<EnglishPegNode> {
        // TODO - if this node value is already computed, return None.
        if self.move_index < self.legal_moves.len() {
            let current_move = self.legal_moves[self.move_index];
            let next_board = board_after_move(&self.state.board, current_move);
            let next_legal_moves = get_legal_moves(&next_board);
            let next_state = EnglishPegState {
                board: next_board
            };

            Some(EnglishPegNode {
                state: next_state,
                legal_moves: next_legal_moves,
                move_index: 0
            })
        } else {
            None
        }
    }

    fn on_children_completed(&mut self) -> BinaryOutcome {
        // TODO - if know the value of this state, put it in the value service
        if count_pegs(&self.state.board) == 1 {
            BinaryOutcome::Win
        } else {
            BinaryOutcome::Lose
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_english_peg_search() {
        let state = EnglishPegState::new();
        let legal_moves = get_legal_moves(&state.board);

        let root_node = EnglishPegNode {
            state,
            legal_moves,
            move_index: 0
        };

        let mut tree = GenerativeEvaluationTree::<EnglishPegNode, BinaryOutcome>::new(
            root_node,
            100,
            Some(BinaryOutcome::Win)
        );

        let search_results = tree.search();
        let action_list: Vec<&EnglishPegMove> = search_results.iter().map(|x| {
            &x.legal_moves[x.move_index]
        }).collect();

        println!("action list {:?}", action_list);
    }

}