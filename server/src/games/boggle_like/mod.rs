pub mod trie;
pub mod direction;

use std::collections::VecDeque;
use trie::{WordTrie, TrieSearchOutcome};
use direction::Direction;

trait LetterGrid {
    fn get_cell_value(&self, col: usize, row: usize) -> u8;
    fn grid_cells(&self) -> Vec<GridCell>;
    fn neighbor_in_direction(&self, cell: &GridCell, direction: Direction) -> Option<GridCell>;
    fn first_cw_neighbor(&self, from: &GridCell, start_direction: Direction) -> GridCell;
    fn first_cw_neighbor_between(&self, from: &GridCell, start_exclusive: Direction, end_exclusive: Direction) -> Option<GridCell>;
    fn direction_to(&self, from: &GridCell, to: &GridCell) -> Option<Direction>;
}

struct LetterGrid4x4 {
    grid: [u8; 16],
}

impl LetterGrid4x4 {
    pub fn new(grid: [u8; 16]) -> LetterGrid4x4 {
        LetterGrid4x4 {
            grid,
        }
    }

    pub fn to_grid_cell(index: usize) -> GridCell {
        GridCell {
            col: index % 4,
            row: index / 4,
        }
    }
}

impl LetterGrid for LetterGrid4x4 {
    
    fn get_cell_value(&self, col: usize, row: usize) -> u8 {
        self.grid[(row * 4) + col]
    }

    fn grid_cells(&self) -> Vec<GridCell> {
        (0..self.grid.len())
            .map(|x| LetterGrid4x4::to_grid_cell(x))
            .collect::<Vec<GridCell>>()
    }

    fn neighbor_in_direction(&self, cell: &GridCell, direction: Direction) -> Option<GridCell> {
        println!("from: {:?}, towards: {:?}", cell, direction);
        let (d_col, d_row) = Direction::direction_vector(&direction);
        println!("vector {:?}", (d_col, d_row));
        let neighbor_col = cell.col as i8 + d_col;
        let neighbor_row = cell.row as i8 + d_row;

        println!("neighbor cell {:?}", (neighbor_col, neighbor_row));

        if neighbor_col > -1 && 
           neighbor_col < 4 &&
           neighbor_row > -1 &&
           neighbor_row < 4
        {
            println!("valid");
            Some(GridCell {
                col: neighbor_col as usize,
                row: neighbor_row as usize,
            })
        } else {
            println!("invalid");
            None
        }
    }

    fn first_cw_neighbor(&self, from: &GridCell, start_direction: Direction) -> GridCell {
        let end_direction = Direction::next_counter_clockwise(&start_direction);

        Direction::directions_between_inclusive_cw(
            &start_direction,
            &end_direction,
        ).into_iter()
            .map(|d| self.neighbor_in_direction(from, d))
            .find(|c| c.is_some())
            .flatten()
            .unwrap()
    }

    fn first_cw_neighbor_between(
        &self,
        from: &GridCell,
        start_exclusive: Direction,
        end_exclusive: Direction,
    ) -> Option<GridCell> {
        Direction::directions_between_exclusive_cw(
            &start_exclusive,
            &end_exclusive,
        ).into_iter()
            .map(|d| self.neighbor_in_direction(from, d))
            .find(|c| c.is_some())
            .flatten()
    }

    fn direction_to(
        &self,
        from: &GridCell,
        to: &GridCell,
    ) -> Option<Direction> {
        let dx = to.col as i8 - from.col as i8;
        let dy = to.row as i8 - from.row as i8;

        Direction::from_direction_vector(&(dx, dy))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GridCell {
    col: usize,
    row: usize,
}

const TILE_NEIGHBORS_CW_4X4: [usize; 84] = [
    // row 1
    1, 5, 4,
    2, 6, 5, 4, 0,
    3, 7, 6, 5, 1,
    7, 6, 2,
    // row2
    0, 1, 5, 9, 8,
    1, 2, 6, 10, 9, 8, 4, 0,
    2, 3, 7, 11, 10, 9, 5, 1,
    3, 11, 10, 6, 2,
    // row3
    4, 5, 9, 13, 12,
    5, 6, 10, 14, 13, 12, 8, 4,
    6, 7, 11, 15, 14, 13, 9, 5,
    7, 15, 14, 10, 6,
    // row 4
    8, 9, 13,
    9, 10, 14, 12, 8,
    10, 11, 15, 13, 9,
    11, 14, 10,
];
const TILE_NEIGHBORS_4X4_OFFSETS: [usize; 16] = [
    0,
    3,
    8,
    13,
    16,
    21,
    29,
    37,
    42,
    47,
    55,
    63,
    68,
    71,
    76,
    81,
];
const TILE_NEIGHBORS_4X4_LENGTHS: [usize; 16] = [
    3,
    5,
    5,
    3,
    5,
    8,
    8,
    5,
    5,
    8,
    8,
    5,
    3,
    5,
    5,
    3,
];

pub trait BoggleLikeAnalyst {
    fn find_all_valid_words(
        &mut self,
        grid: &dyn LetterGrid,
        dictionary: &mut WordTrie,
    );

    fn find_valid_words_starting_with_cell(
        &mut self,
        cell: &GridCell,
        grid: &dyn LetterGrid,
        dictionary: &mut WordTrie,
    );

    
}

pub struct State4x4Analyst {
    valid_words: Vec<Vec<GridCell>>,
}

impl State4x4Analyst {
    pub fn new() -> State4x4Analyst {
        State4x4Analyst {
            valid_words: vec![],
        }
    }

    fn search_for_word_from_path(
        path_stack: &Vec<GridCell>,
        letter_grid: &dyn LetterGrid,
        dictionary: &mut WordTrie,
    ) -> TrieSearchOutcome {
        let word: Vec<u8> = path_stack.iter()
            .map(|cell| letter_grid.get_cell_value(cell.col, cell.row))
            .collect();
        
        println!("word for path: {:?}", word);
        
        dictionary.find(word.as_slice())
    }

    fn collect_word(
        &mut self,
        path_stack: &Vec<GridCell>,
    ) {
        let path_copy: Vec<GridCell> = path_stack.iter()
            .map(|i| *i)
            .collect();
        self.valid_words.push(path_copy);
    }

    fn next_path(
        path_stack: &mut Vec<GridCell>,
        grid: &dyn LetterGrid,
        has_longer_words: bool,
    ) {
        if has_longer_words {
            State4x4Analyst::next_forward_path(
                path_stack,
                grid,
            );
        } else {
            State4x4Analyst::next_backtracked_path(
                path_stack,
                grid,
            );
        }
    }

    fn next_forward_path(
        path_stack: &mut Vec<GridCell>,
        grid: &dyn LetterGrid,
    ) {
        let center_cell = path_stack.last().unwrap();
        let next_cell = grid.first_cw_neighbor(center_cell, Direction::North);
        println!("next_cell {:?}", next_cell);
        if !path_stack.contains(&next_cell) {
            path_stack.push(next_cell);
        }
    }

    fn next_backtracked_path(
        path_stack: &mut Vec<GridCell>,
        grid: &dyn LetterGrid,
    ) {
        let last_cell = path_stack.pop().unwrap();

        if path_stack.is_empty() {
            ()
        } else {
            let center_cell = path_stack.last().unwrap();
            let start_direction = grid.direction_to(center_cell, &last_cell).unwrap();
            let maybe_next_cell = grid.first_cw_neighbor_between(
                center_cell,
                start_direction,
                Direction::North,
            );

            match maybe_next_cell {
                Some(c) => {
                    if !path_stack.contains(&c) {
                        path_stack.push(c);
                    }
                },
                None => {},
            }
        }
    }
}

impl BoggleLikeAnalyst for State4x4Analyst {
    fn find_all_valid_words(
        &mut self,
        grid: &dyn LetterGrid,
        dictionary: &mut WordTrie,
    ) {
        for cell in grid.grid_cells() {
            println!("root cell: {:?}", cell);
            self.find_valid_words_starting_with_cell(
                &cell,
                grid,
                dictionary,
            );
        }
    }

    fn find_valid_words_starting_with_cell(
        &mut self,
        cell: &GridCell,
        grid: &dyn LetterGrid,
        dictionary: &mut WordTrie,
    ) {
        let mut path_stack: Vec<GridCell> = vec![*cell];
        while !path_stack.is_empty() {
            println!("current path: {:?}", path_stack);
            let search_outcome = State4x4Analyst::search_for_word_from_path(
                &path_stack,
                grid,
                dictionary,
            );

            if search_outcome.is_word {
                println!("is word!");
                self.collect_word(&path_stack);
            }

            State4x4Analyst::next_path(
                &mut path_stack,
                grid,
                search_outcome.has_longer_words,
            );
        }
    }    
}

fn letter_code_from_alphabet_index(index: usize) -> u8 {
    index as u8 + 97
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_letter_grid() -> LetterGrid4x4 {
        LetterGrid4x4 {
            grid: [
                letter_code_from_alphabet_index(0),
                letter_code_from_alphabet_index(15),
                letter_code_from_alphabet_index(15),
                letter_code_from_alphabet_index(11),
                letter_code_from_alphabet_index(25),
                letter_code_from_alphabet_index(6),
                letter_code_from_alphabet_index(6),
                letter_code_from_alphabet_index(4),
                letter_code_from_alphabet_index(4),
                letter_code_from_alphabet_index(25),
                letter_code_from_alphabet_index(25),
                letter_code_from_alphabet_index(19),
                letter_code_from_alphabet_index(3),
                letter_code_from_alphabet_index(8),
                letter_code_from_alphabet_index(19),
                letter_code_from_alphabet_index(4),
            ],
        }
    }

    #[test]
    fn test_neighbor_in_direction() {
        let letter_grid = create_test_letter_grid();

        let sut: Vec<(Direction, Option<GridCell>)> = vec![
            (Direction::North, None),
            (Direction::NorthEast, None),
            (Direction::East, Some(GridCell { col: 1, row: 0})),
            (Direction::SouthEast, Some(GridCell { col: 1, row: 1})),
            (Direction::South, Some(GridCell { col: 0, row: 1})),
            (Direction::SouthWest, None),
            (Direction::West, None),
            (Direction::NorthWest, None),
        ];

        for (d, expected_neighbor) in sut {
            let actual_neighbor = letter_grid.neighbor_in_direction(
                &GridCell { col: 0, row: 0 },
                d,
            );

            assert_eq!(actual_neighbor, expected_neighbor);
        }
        
    }

    #[test]
    fn test_first_cw_neighbor() {
        let letter_grid = create_test_letter_grid();

        let expected_neighbor = GridCell { col: 1, row: 0 };

        let actual_neighbor = letter_grid.first_cw_neighbor(
            &GridCell { col: 0, row: 0},
            Direction::North,
        );

        assert_eq!(actual_neighbor, expected_neighbor);
    }

    #[test]
    fn test_next_forward_path() {
        let letter_grid = create_test_letter_grid();

        let mut path_stack: Vec<GridCell> = vec![
            GridCell { col: 0, row: 0 },
        ];

        let expected_next_path: Vec<GridCell> = vec![
            GridCell { col: 0, row: 0 },
            GridCell { col: 1, row: 0 },
        ];

        State4x4Analyst::next_forward_path(
            &mut path_stack,
            &letter_grid,
        );

        assert_eq!(path_stack, expected_next_path);
    }

    // #[test]
    // fn test_find_all_valid_words() {
    //     let state = LetterGrid4x4 {
    //         grid: [
    //             letter_code_from_alphabet_index(0),
    //             letter_code_from_alphabet_index(15),
    //             letter_code_from_alphabet_index(15),
    //             letter_code_from_alphabet_index(11),
    //             letter_code_from_alphabet_index(25),
    //             letter_code_from_alphabet_index(6),
    //             letter_code_from_alphabet_index(6),
    //             letter_code_from_alphabet_index(4),
    //             letter_code_from_alphabet_index(4),
    //             letter_code_from_alphabet_index(25),
    //             letter_code_from_alphabet_index(25),
    //             letter_code_from_alphabet_index(19),
    //             letter_code_from_alphabet_index(3),
    //             letter_code_from_alphabet_index(8),
    //             letter_code_from_alphabet_index(19),
    //             letter_code_from_alphabet_index(4),
    //         ],
    //     };

    //     let mut dictionary = WordTrie::from_words(&[
    //         "app",
    //         "apple",
    //         "let",
    //         "egg",
    //         "leg",
    //         "did",
    //         "daze",
    //         "tide",
    //         "lets",
    //         "gel",
    //     ]);
        
    //     let result = dictionary.find("app".as_bytes());
    //     println!("result is: {:?}", result);

    //     let mut analyst = State4x4Analyst::new();
    //     analyst.find_all_valid_words(&state, &mut dictionary);

    //     let expected_valid_words: Vec<Vec<GridCell>> = vec![
    //         vec![0, 1, 2], // app
    //         vec![0, 1, 2, 3, 7], // apple
    //         vec![3, 7, 11], // let
    //         vec![3, 7, 11, 10], // lets
    //         vec![3, 7, 6], // leg
    //         vec![6, 7, 3], // gel
    //         vec![7, 6, 5], // egg
    //     ].into_iter()
    //         .map(|cells| {
    //             cells.into_iter()
    //                 .map(|i| LetterGrid4x4::to_grid_cell(i))
    //                 .collect()
    //         })
    //         .collect();
        
    //     assert_eq!(analyst.valid_words, expected_valid_words);
    // }
}