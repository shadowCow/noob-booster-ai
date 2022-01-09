pub mod trie;

use std::collections::VecDeque;
use trie::WordTrie;

pub struct State4x4 {
    letter_grid: [char; 16],
}

impl State4x4 {
    pub fn new(letter_grid: [char; 16]) -> State4x4 {
        State4x4 {
            letter_grid,
        }
    }
}

pub trait BoggleLikeAnalyst {
    fn neighbors_of(&self, tile: usize) -> &[usize];
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
        dictionary: &WordTrie,
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
        dictionary: &WordTrie,
    ) {
        let mut visited_cells: [bool; 16] = [false; 16];
        visited_cells[cell] = true;

        let mut next_cells: VecDeque<usize> = VecDeque::from(vec![cell]);
        while let Some(next_cell) = next_cells.pop_front() {
            
        }
    }
}

impl BoggleLikeAnalyst for State4x4Analyst {
    fn neighbors_of(&self, tile: usize) -> &[usize] {
        let start_index = TILE_NEIGHBORS_4X4_OFFSETS[tile];
        let end_index = start_index + TILE_NEIGHBORS_4X4_LENGTHS[tile];

        &TILE_NEIGHBORS_4X4[start_index .. end_index]
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