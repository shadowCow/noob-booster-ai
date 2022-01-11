pub mod trie;

use std::collections::VecDeque;
use trie::WordTrie;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn direction_vector(direction: &Direction) -> (i8, i8) {
        match direction {
            Direction::North => (0, 1),
            Direction::NorthEast => (1, 1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, -1),
            Direction::South => (0, -1),
            Direction::SouthWest => (-1, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, 1),
        }
    }

    pub fn next_clockwise(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }
}

pub struct State4x4 {
    letter_grid: [u8; 16],
}

impl State4x4 {
    pub fn new(letter_grid: [u8; 16]) -> State4x4 {
        State4x4 {
            letter_grid,
        }
    }
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
    fn neighbors_of(&self, tile: usize) -> &[usize];
    fn neighbor_in_direction(&self, tile: usize, direction: (i8, i8)) -> Option<usize>;
    fn next_neighbor_clockwise(&self, tile: usize, direction: &Direction) -> usize;
}

pub struct State4x4Analyst {
    valid_words: Vec<Vec<usize>>,
}

impl State4x4Analyst {
    pub fn new() -> State4x4Analyst {
        State4x4Analyst {
            valid_words: vec![],
        }
    }

    pub fn find_all_valid_words(
        &mut self,
        state: &State4x4,
        dictionary: &mut WordTrie,
    ) {
        for cell in 0..state.letter_grid.len() {
            println!("root cell: {:?}", cell);
            self.find_valid_words_for_cell(
                cell,
                state,
                dictionary,
            );
        }
    }

    fn find_valid_words_for_cell(
        &mut self,
        cell: usize,
        state: &State4x4,
        dictionary: &mut WordTrie,
    ) {
        let mut visited_cells: [bool; 16] = [false; 16];
        visited_cells[cell] = true;

        let mut path_stack: Vec<usize> = vec![cell];
        while !path_stack.is_empty() {
            println!("current path: {:?}", path_stack);
            // check for word
            let word: Vec<u8> = path_stack.iter()
                .map(|i| state.letter_grid[*i])
                .collect();
            println!("word for path: {:?}", word);
            let search_outcome = dictionary.find(word.as_slice());

            if search_outcome.is_word {
                println!("is word!");
                let path_copy: Vec<usize> = path_stack.iter()
                    .map(|i| *i)
                    .collect();
                self.valid_words.push(path_copy);
            }

            if !search_outcome.has_longer_words {
                println!("no longer words");
                let last_cell = path_stack.pop();
                visited_cells[last_cell.unwrap()] = false;
            }

            self.next_path(
                &mut path_stack,
                &mut visited_cells,
            );
        }
    }

    fn next_path(
        &self,
        path_stack: &mut Vec<usize>,
        visited_cells: &mut [bool; 16],
    ) {
        if !path_stack.is_empty() {
            let mut maybe_next_neighbor = self.neighbors_of(*path_stack.last().unwrap())
                .iter()
                .find(|x| !visited_cells[**x]);
            println!("maybe_next_neighbor: {:?}", maybe_next_neighbor);
            
            while !path_stack.is_empty() {
                match maybe_next_neighbor {
                    Some(n) => {
                        path_stack.push(*n);
                        visited_cells[*n] = true;
                        break;
                    },
                    None => {
                        // backtrack and try that cell
                        let last_cell = path_stack.pop();
                        visited_cells[last_cell.unwrap()] = false;
                        // this is the problem - we arent tracking visited properly as
                        // we backtrack.  when we backtrack, we need to go to the
                        // the next cw unvisited neighbor of the last cell
                        // so we cant just unvisit this before finding that cell

                        if !path_stack.is_empty() {
                            maybe_next_neighbor = self.neighbors_of(*path_stack.last().unwrap())
                                .iter()
                                .find(|x| !visited_cells[**x]);
                        } else {
                            maybe_next_neighbor = None;
                        }
                    },
                }    
            }
        }
    }
}

impl BoggleLikeAnalyst for State4x4Analyst {
    fn neighbors_of(&self, tile: usize) -> &[usize] {
        let start_index = TILE_NEIGHBORS_4X4_OFFSETS[tile];
        let end_index = start_index + TILE_NEIGHBORS_4X4_LENGTHS[tile];

        &TILE_NEIGHBORS_CW_4X4[start_index .. end_index]
    }
    
    fn neighbor_in_direction(&self, tile: usize, direction: (i8, i8)) -> Option<usize> {
        let col = tile % 4;
        let row = tile / 4;

        let target_col = col as i8 + direction.0;
        let target_row = row as i8 + direction.1;

        if target_col < 0 ||
           target_row < 0 ||
           target_col > 3 ||
           target_row > 3
        {
            None
        } else {
            let index = (target_row * 4) + target_col;
            
            Some(index as usize)
        }
    }

    fn next_neighbor_clockwise(&self, tile: usize, direction: &Direction) -> usize {
        let mut neighbor = self.neighbor_in_direction(
            tile,
            Direction::direction_vector(direction),
        );
        while neighbor.is_none() {
            neighbor = self.neighbor_in_direction(
                tile,
                Direction::direction_vector(&Direction::next_clockwise(direction)),
            );
        }

        neighbor.unwrap()
    }
}

fn letter_code_from_alphabet_index(index: usize) -> u8 {
    index as u8 + 97
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_neighbors_of() {
        let analyst = State4x4Analyst::new();

        let n12_expected = &[8, 9, 13];
        let n12_actual = analyst.neighbors_of(12);
        assert_eq!(n12_actual, n12_expected);

        let n6_expected = &[2, 3, 7, 11, 10, 9, 5, 1];
        let n6_actual = analyst.neighbors_of(6);
        assert_eq!(n6_actual, n6_expected);
    }

    #[test]
    fn test_find_all_valid_words() {
        let state = State4x4 {
            letter_grid: [
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
        };

        let mut dictionary = WordTrie::from_words(&[
            "app",
            "apple",
            "let",
            "egg",
            "leg",
            "did",
            "daze",
            "tide",
            "lets",
            "gel",
        ]);
        
        let result = dictionary.find("app".as_bytes());
        println!("result is: {:?}", result);

        let mut analyst = State4x4Analyst::new();
        analyst.find_all_valid_words(&state, &mut dictionary);

        let expected_valid_words: Vec<Vec<usize>> = vec![
            vec![0, 1, 2], // app
            vec![0, 1, 2, 3, 7], // apple
            vec![3, 7, 11], // let
            vec![3, 7, 11, 10], // lets
            vec![3, 7, 6], // leg
            vec![6, 7, 3], // gel
            vec![7, 6, 5], // egg
        ];
        
        assert_eq!(analyst.valid_words, expected_valid_words);
    }
}