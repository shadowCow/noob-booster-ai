pub mod trie;
pub mod direction;

use std::collections::VecDeque;
use trie::{WordTrie, TrieSearchOutcome};
use direction::Direction;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

trait LetterGrid {
    fn get_cell_value(&self, col: usize, row: usize) -> u8;
    fn grid_cells(&self) -> Vec<GridCell>;
    fn neighbor_in_direction(&self, cell: &GridCell, direction: Direction) -> Option<GridCell>;
    fn first_cw_neighbor(
        &self,
        from: &GridCell,
        start_direction: Direction,
        blacklist: &Vec<GridCell>,
    ) -> Option<GridCell>;
    fn first_cw_neighbor_between(
        &self,
        from: &GridCell,
        start_exclusive: Direction,
        end_exclusive: Direction,
        blacklist: &Vec<GridCell>,
    ) -> Option<GridCell>;
    fn direction_to(&self, from: &GridCell, to: &GridCell) -> Option<Direction>;
    fn contains_letter(&self, letter: u8) -> bool;
}

struct LetterGrid4x4 {
    grid: [u8; 16],
    letters_present: [bool; 26], // 26 element vec, each element is true if puzzle contains letter, false otherwise
}

impl LetterGrid4x4 {
    pub fn new(grid: [u8; 16]) -> LetterGrid4x4 {
        let mut letters_present = [false; 26];
        grid.iter()
            .map(|letter| letter_code_to_alphabet_index(*letter))
            .for_each(|index| letters_present[index] = true);
        
        LetterGrid4x4 {
            grid,
            letters_present,
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
    fn contains_letter(&self, letter: u8) -> bool {
        let index = letter_code_to_alphabet_index(letter);
        self.letters_present[index]
    }

    fn get_cell_value(&self, col: usize, row: usize) -> u8 {
        self.grid[(row * 4) + col]
    }

    fn grid_cells(&self) -> Vec<GridCell> {
        (0..self.grid.len())
            .map(|x| LetterGrid4x4::to_grid_cell(x))
            .collect::<Vec<GridCell>>()
    }

    fn neighbor_in_direction(&self, cell: &GridCell, direction: Direction) -> Option<GridCell> {
        let (d_col, d_row) = Direction::direction_vector(&direction);
        
        let neighbor_col = cell.col as i8 + d_col;
        let neighbor_row = cell.row as i8 + d_row;

        if neighbor_col > -1 && 
           neighbor_col < 4 &&
           neighbor_row > -1 &&
           neighbor_row < 4
        {
            Some(GridCell {
                col: neighbor_col as usize,
                row: neighbor_row as usize,
            })
        } else {
            None
        }
    }

    fn first_cw_neighbor(
        &self,
        from: &GridCell,
        start_direction: Direction,
        blacklist: &Vec<GridCell>,
    ) -> Option<GridCell> {
        let end_direction = Direction::next_counter_clockwise(&start_direction);

        Direction::directions_between_inclusive_cw(
            &start_direction,
            &end_direction,
        ).into_iter()
            .map(|d| self.neighbor_in_direction(from, d))
            .filter(|c| c.filter(|x| !blacklist.contains(x)).is_some())
            .find(|c| c.is_some())
            .flatten()
    }

    fn first_cw_neighbor_between(
        &self,
        from: &GridCell,
        start_exclusive: Direction,
        end_exclusive: Direction,
        blacklist: &Vec<GridCell>,
    ) -> Option<GridCell> {
        Direction::directions_between_exclusive_cw(
            &start_exclusive,
            &end_exclusive,
        ).into_iter()
            .map(|d| self.neighbor_in_direction(from, d))
            .filter(|c| c.filter(|x| !blacklist.contains(x)).is_some())
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

impl BoggleLikeAnalyst for State4x4Analyst {
    fn find_all_valid_words(
        &mut self,
        grid: &dyn LetterGrid,
        dictionary: &mut WordTrie,
    ) {
        for cell in grid.grid_cells() {
            // println!("root cell: {:?}", cell);
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
        /*
        while let Some(current_path) = pathfinder.next_path() {
            let search_outcome = trie.search(word_for(current_path));
            if search_outcome.is_word {
                collect_word(...)
            }
        }
        */
        let mut path_stack: Vec<GridCell> = vec![*cell];
        while !path_stack.is_empty() {
            // println!("current path: {:?}", path_stack);
            // println!("current word: {:?}", to_word(&path_stack, grid));
            let search_outcome = State4x4Analyst::search_for_word_from_path(
                &path_stack,
                grid,
                dictionary,
            );
            // println!("search_outcome: {:?}", search_outcome);

            if search_outcome.is_word {
                // println!("is word - {:?}", to_word(&path_stack, grid));

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
        
        // println!("word for path: {:?}", word);
        
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
        
        let maybe_next_cell = grid.first_cw_neighbor(
            center_cell,
            Direction::North,
            path_stack,
        );
        // println!("looking forward from: {:?} to {:?}", center_cell, maybe_next_cell);
        match maybe_next_cell {
            Some(next_cell) => path_stack.push(next_cell),
            None => {
                State4x4Analyst::next_backtracked_path(
                    path_stack,
                    grid,
                );
            },
        }
    }

    fn next_backtracked_path(
        path_stack: &mut Vec<GridCell>,
        grid: &dyn LetterGrid,
    ) {
        // println!("looking backward");
        while !path_stack.is_empty() {
            let found_next_path = State4x4Analyst::backtrack_one(path_stack, grid);
            if found_next_path {
                break;
            }
        }
    }

    fn backtrack_one(
        path_stack: &mut Vec<GridCell>,
        grid: &dyn LetterGrid,
    ) -> bool {
        let mut last_cell = path_stack.pop().unwrap();
        // println!("backtracking one from {:?}", last_cell);
        if path_stack.is_empty() {
            // println!("empty path stack");
            false
        } else {
            let center_cell = path_stack.last().unwrap();
            // println!("backtrack center {:?}", center_cell);
            let start_direction = grid.direction_to(center_cell, &last_cell).unwrap();
            
            let maybe_next_cell = grid.first_cw_neighbor_between(
                center_cell,
                start_direction,
                Direction::North,
                path_stack,
            );
            // println!("maybe_next_cell {:?}", maybe_next_cell);
    
            match maybe_next_cell {
                Some(c) => {
                    path_stack.push(c);
                    true
                },
                None => false,
            }
        }
    }
}

fn dictionary_for_grid(
    word_list_file_path: &str,
    grid: &dyn LetterGrid,
    min_word_length: usize,
) -> io::Result<WordTrie> {
    let mut all_words: Vec<String> = vec![];

    let lines = read_lines(word_list_file_path)?;

    for line_result in lines {
        if let Ok(line) = line_result {
            if line.len() >= min_word_length &&
                line.as_bytes().iter().all(|b| grid.contains_letter(*b))
            {
                // println!("adding line {:?}", line);
                all_words.push(line);
            }
        }
    }

    Ok(WordTrie::from_words_owned(all_words.as_slice()))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn letter_code_from_alphabet_index(index: usize) -> u8 {
    index as u8 + 65
}

fn letter_code_to_alphabet_index(letter_code: u8) -> usize {
    (letter_code - 65) as usize
}

fn to_words(
    paths: &Vec<Vec<GridCell>>,
    grid: &dyn LetterGrid,
) -> Vec<String> {
    paths.iter()
        .map(|p| to_word(p, grid))
        .collect()
}

fn to_word(
    cells: &Vec<GridCell>,
    grid: &dyn LetterGrid,
) -> String {
    let bytes: Vec<u8> = cells.iter()
        .map(|cell| grid.get_cell_value(cell.col, cell.row))
        .collect();
    
    String::from_utf8(bytes).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_letter_grid() -> LetterGrid4x4 {
        /*
        0  1  2  3
        4  5  6  7
        8  9  10 11
        12 13 14 15


        A P P L
        M G G E
        Z Z S T
        D I T E
        */
        LetterGrid4x4::new([
            letter_code_from_alphabet_index(0), // a
            letter_code_from_alphabet_index(15), // p
            letter_code_from_alphabet_index(15), // p
            letter_code_from_alphabet_index(11), // l
            letter_code_from_alphabet_index(12), // m
            letter_code_from_alphabet_index(6), // g
            letter_code_from_alphabet_index(6), // g
            letter_code_from_alphabet_index(4), // e
            letter_code_from_alphabet_index(25), // z
            letter_code_from_alphabet_index(25), // z
            letter_code_from_alphabet_index(18), // s 
            letter_code_from_alphabet_index(19), // t
            letter_code_from_alphabet_index(3), // d
            letter_code_from_alphabet_index(8), // i
            letter_code_from_alphabet_index(19), // t
            letter_code_from_alphabet_index(4), // e
        ])
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
    fn test_first_cw_neighbor_between() {
        let letter_grid = create_test_letter_grid();

        let expected_neighbor = Some(GridCell { col: 2, row: 1 });

        let actual_neighbor = letter_grid.first_cw_neighbor_between(
            &GridCell { col: 3, row: 0},
            Direction::South,
            Direction::North,
            &vec![],
        );

        assert_eq!(actual_neighbor, expected_neighbor);
    }

    #[test]
    fn test_first_cw_neighbor() {
        let letter_grid = create_test_letter_grid();

        let expected_neighbor = Some(GridCell { col: 1, row: 0 });

        let actual_neighbor = letter_grid.first_cw_neighbor(
            &GridCell { col: 0, row: 0},
            Direction::North,
            &vec![],
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

    #[test]
    fn test_next_backtracked_path() {
        let letter_grid = create_test_letter_grid();

        let mut path_stack: Vec<GridCell> = vec![
            GridCell { col: 0, row: 0 },
            GridCell { col: 1, row: 0 },
            GridCell { col: 2, row: 0 },
            GridCell { col: 3, row: 0 },
            GridCell { col: 3, row: 1 },
        ];

        let expected_next_path: Vec<GridCell> = vec![
            GridCell { col: 0, row: 0 },
            GridCell { col: 1, row: 0 },
            GridCell { col: 2, row: 0 },
            GridCell { col: 3, row: 0 },
            GridCell { col: 2, row: 1 },
        ];

        State4x4Analyst::next_backtracked_path(
            &mut path_stack,
            &letter_grid,
        );

        assert_eq!(path_stack, expected_next_path);
    }

    #[test]
    fn test_find_all_valid_words() {
        let grid = create_test_letter_grid();

        let grid_as_letters: Vec<String> = grid.grid.iter().map(|b| String::from_utf8(vec![*b]).unwrap()).collect();
        println!("starting grid {:?}", grid_as_letters);

        let mut dictionary = WordTrie::from_words(&[
            "APP",
            "APPLE",
            "LET",
            "EGG",
            "LEG",
            "DID",
            "DAZE",
            "TIDE",
            "LETS",
            "GEL",
        ]);
        
        let mut analyst = State4x4Analyst::new();
        analyst.find_all_valid_words(&grid, &mut dictionary);

        let expected_valid_words: Vec<Vec<GridCell>> = vec![
            vec![0, 1, 2], // app
            vec![0, 1, 2, 3, 7], // apple
            vec![3, 7, 11], // let
            vec![3, 7, 11, 10], // lets
            vec![3, 7, 6], // leg
            vec![6, 7, 3], // gel
            vec![7, 6, 5], // egg
        ].into_iter()
            .map(|cells| {
                cells.into_iter()
                    .map(|i| LetterGrid4x4::to_grid_cell(i))
                    .collect()
            })
            .collect();

        // let found_words = to_words(&analyst.valid_words, &grid);
        // println!("found words {:?}", found_words);
        
        assert_eq!(analyst.valid_words, expected_valid_words);
    }

    #[test]
    fn test_dictionary_for_grid() {
        let grid = create_test_letter_grid();

        let file_path = "/Users/dwadeson/Downloads/collins_scrabble_words_2019.txt";

        let maybe_dictionary = dictionary_for_grid(
            file_path,
            &grid,
            3,
        );

        assert_eq!(maybe_dictionary.is_ok(), true);

        // let mut dictionary = maybe_dictionary.unwrap();

        // let mut analyst = State4x4Analyst::new();
        // analyst.find_all_valid_words(&grid, &mut dictionary);

        // let found_words = to_words(&analyst.valid_words, &grid);
        // println!("found words {:?}", found_words);
    }
}