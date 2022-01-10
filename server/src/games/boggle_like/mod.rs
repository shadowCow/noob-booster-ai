pub mod trie;

use std::collections::VecDeque;
use trie::WordTrie;

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

const TILE_NEIGHBORS_4X4: [usize; 84] = [
    // row 1
    1, 4, 5,
    0, 2, 4, 5, 6,
    1, 3, 5, 6, 7,
    2, 6, 7,
    // row2
    0, 1, 5, 8, 9,
    0, 1, 2, 4, 6, 8, 9, 10,
    1, 2, 3, 5, 7, 9, 10, 11,
    2, 3, 6, 10, 11,
    // row3
    4, 5, 9, 12, 13,
    4, 5, 6, 8, 10, 12, 13, 14,
    5, 6, 7, 9, 11, 13, 14, 15,
    6, 7, 10, 14, 15,
    // row 4
    8, 9, 13,
    8, 9, 10, 12, 14,
    9, 10, 11, 13, 15,
    10, 11, 14,
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

        let mut path_stack: Vec<usize> = vec![];
        while !path_stack.is_empty() {
            // check for word
            let word: Vec<u8> = path_stack.iter()
                .map(|i| state.letter_grid[*i])
                .collect();
            let search_outcome = dictionary.find(word.as_slice());

            if search_outcome.is_word {
                let path_copy: Vec<usize> = path_stack.iter()
                    .map(|i| *i)
                    .collect();
                self.valid_words.push(path_copy);
            }

            // set next path
            if search_outcome.has_longer_words {
                // extend stack with next unvisited neighbor
            } else {
                // unwind stack to last node with unvisited neighbors
            }
        }
    }

    // fn next_unvisited_neighbor(
    //     visited_cells: &[bool; 16],
    //     current_cell: usize,
    // ) -> usize {
    //     let 
    // }

    // fn backtrack_to_first_unvisited_neighbor(
        
    // )
}

impl BoggleLikeAnalyst for State4x4Analyst {
    fn neighbors_of(&self, tile: usize) -> &[usize] {
        let start_index = TILE_NEIGHBORS_4X4_OFFSETS[tile];
        let end_index = start_index + TILE_NEIGHBORS_4X4_LENGTHS[tile];

        &TILE_NEIGHBORS_4X4[start_index .. end_index]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_neighbors_of() {
        let analyst = State4x4Analyst::new();

        let n12_expected = &[8, 9, 13];
        let n12_actual = analyst.neighbors_of(12);
        assert_eq!(n12_actual, n12_expected);

        let n6_expected = &[1, 2, 3, 5, 7, 9, 10, 11];
        let n6_actual = analyst.neighbors_of(6);
        assert_eq!(n6_actual, n6_expected);
    }
}