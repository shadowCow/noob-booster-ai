use std::collections::HashMap;
use std::hash::Hash;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

trait WordIndex {
    fn list_matching_words(&self, query: QueryNode) -> Vec<String>;
}

#[derive(Debug)]
enum QueryNode {
    Op(Box<QueryOp>),
    Location(LetterLocation),
}

#[derive(Debug)]
struct QueryOp {
    kind: QueryOpKind,
    left: QueryNode,
    right: QueryNode,
}

fn AndNode(left: QueryNode, right: QueryNode) -> QueryNode {
    QueryNode::Op(Box::new(QueryOp {
        kind: QueryOpKind::And,
        left,
        right,
    }))
}

fn OrNode(left: QueryNode, right: QueryNode) -> QueryNode {
    QueryNode::Op(Box::new(QueryOp {
        kind: QueryOpKind::Or,
        left,
        right,
    }))
}

fn FixedNode(letter: char, slot: usize) -> QueryNode {
    QueryNode::Location(LetterLocation {
        letter: Letter::from_char(letter).unwrap(),
        slot: LetterSlot::from_usize(slot - 1).unwrap(),
    })
}

#[derive(Debug)]
enum QueryOpKind {
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct LetterLocation {
    letter: Letter,
    slot: LetterSlot, 
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum LetterSlot {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl LetterSlot {
    fn from_usize(x: usize) -> Result<LetterSlot, LetterSlotError> {
        match x {
            0 => Ok(LetterSlot::One),
            1 => Ok(LetterSlot::Two),
            2 => Ok(LetterSlot::Three),
            3 => Ok(LetterSlot::Four),
            4 => Ok(LetterSlot::Five),
            _ => Err(LetterSlotError { slot_as_usize: x }),
        }
    }

    fn values() -> Vec<LetterSlot> {
        vec![
            LetterSlot::One,
            LetterSlot::Two,
            LetterSlot::Three,
            LetterSlot::Four,
            LetterSlot::Five,
        ]
    }
}

#[derive(Debug)]
struct LetterSlotError {
    slot_as_usize: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Letter {
    fn from_char(x: char) -> Result<Letter, LetterError> {
        match x.to_ascii_lowercase() {
            'a' => Ok(Letter::A),
            'b' => Ok(Letter::B),
            'c' => Ok(Letter::C),
            'd' => Ok(Letter::D),
            'e' => Ok(Letter::E),
            'f' => Ok(Letter::F),
            'g' => Ok(Letter::G),
            'h' => Ok(Letter::H),
            'i' => Ok(Letter::I),
            'j' => Ok(Letter::J),
            'k' => Ok(Letter::K),
            'l' => Ok(Letter::L),
            'm' => Ok(Letter::M),
            'n' => Ok(Letter::N),
            'o' => Ok(Letter::O),
            'p' => Ok(Letter::P),
            'q' => Ok(Letter::Q),
            'r' => Ok(Letter::R),
            's' => Ok(Letter::S),
            't' => Ok(Letter::T),
            'u' => Ok(Letter::U),
            'v' => Ok(Letter::V),
            'w' => Ok(Letter::W),
            'x' => Ok(Letter::X),
            'y' => Ok(Letter::Y),
            'z' => Ok(Letter::Z),
            _ => Err(LetterError { letter_as_char: x }),
        }
    }

    fn values() -> Vec<Letter> {
        vec![
            Letter::A,
            Letter::B,
            Letter::C,
            Letter::D,
            Letter::E,
            Letter::F,
            Letter::G,
            Letter::H,
            Letter::I,
            Letter::J,
            Letter::K,
            Letter::L,
            Letter::M,
            Letter::N,
            Letter::O,
            Letter::P,
            Letter::Q,
            Letter::R,
            Letter::S,
            Letter::T,
            Letter::U,
            Letter::V,
            Letter::W,
            Letter::X,
            Letter::Y,
            Letter::Z,
        ]
    }
}

#[derive(Debug)]
struct LetterError {
    letter_as_char: char
}

#[derive(Debug)]
enum EvaluationNode {
    Pending {
        op: QueryOpKind,
        left: Box<EvaluationNode>,
        right: Box<EvaluationNode>,
    },
    Evaluated(Vec<String>),
}

#[derive(Debug)]
struct WordIndexFive {
    letter_location_map: HashMap<LetterLocation, Vec<String>>,
}

impl WordIndexFive {
    fn new(word_list: Vec<String>) -> WordIndexFive {
        let mut letter_location_map: HashMap<LetterLocation, Vec<String>> = HashMap::new();
        for letter in Letter::values() {
            for slot in LetterSlot::values() {
                letter_location_map.insert(
                    LetterLocation {
                        letter,
                        slot,
                    },
                    vec![],
                );
            }
        }

        for word in word_list {
            for (index, ch) in word.char_indices() {
                let slot = LetterSlot::from_usize(index).unwrap();
                let letter = Letter::from_char(ch).unwrap();

                let key = LetterLocation {
                    letter,
                    slot,
                };

                letter_location_map.get_mut(&key).map(|v| v.push(word.to_owned()));
            }
        }

        WordIndexFive {
            letter_location_map,
        }
    }

    fn evaluate_query(&self, query: QueryNode) -> Vec<String> {
        match query {
            QueryNode::Location(loc) => {
                let vec_copy: Vec<String> = self.letter_location_map
                    .get(&loc)
                    .map(|v| v.to_vec())
                    .unwrap_or(vec![]);

                vec_copy
            },
            QueryNode::Op(op) => {
                let l = self.evaluate_query(op.left);
                let r = self.evaluate_query(op.right);

                match op.kind {
                    QueryOpKind::And => list_intersection(l, r),
                    QueryOpKind::Or => list_union(l, r),
                }
            },
        }
    }
}

impl WordIndex for WordIndexFive {
    fn list_matching_words(&self, query: QueryNode) -> Vec<String> {
        self.evaluate_query(query)
    }
}

fn list_intersection(l1: Vec<String>, l2: Vec<String>) -> Vec<String> {
    let mut combined: Vec<String> = vec![];

    for w in l1 {
        if l2.contains(&w) {
            combined.push(w.to_owned());
        }
    }

    combined
}

fn list_union(l1: Vec<String>, l2: Vec<String>) -> Vec<String> {
    let mut combined: Vec<String> = vec![];

    for w in l1 {
        combined.push(w.to_owned());
    }

    for w in l2 {
        if !combined.contains(&w) {
            combined.push(w.to_owned());
        }
    }

    combined
}


fn create_word_list_from_file(
    word_list_file_path: &str,
) -> io::Result<Vec<String>> {
    let mut word_list: Vec<String> = vec![];
    let lines = read_lines(word_list_file_path)?;

    for line_result in lines {
        if let Ok(line) = line_result {
            word_list.push(line);
        }
    }

    Ok(word_list)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_word_list() -> Vec<String> {
        vec![
            "AAHED".to_owned(),
            "AALII".to_owned(),
            "BAAED".to_owned(),
            "BAALS".to_owned(),
            "CAAED".to_owned(),
            "CABAL".to_owned(),
            "DAALS".to_owned(),
            "DABBA".to_owned(),
            "EAGER".to_owned(),
            "EAGLE".to_owned(),
            "FABBY".to_owned(),
            "FABLE".to_owned(),
            "GABBA".to_owned(),
            "GABBY".to_owned(),
            "HAAFS".to_owned(),
            "HAARS".to_owned(),
            "IAMBI".to_owned(),
            "IAMBS".to_owned(),
            "JAAPS".to_owned(),
            "JABOT".to_owned(),
            "KAAMA".to_owned(),
            "KABAB".to_owned(),
            "LAARI".to_owned(),
            "LABDA".to_owned(),
            "MAAED".to_owned(),
            "MAARE".to_owned(),
            "NAAMS".to_owned(),
            "NAANS".to_owned(),
            "OAKED".to_owned(),
            "OAKEN".to_owned(),
            "PAALS".to_owned(),
            "PAANS".to_owned(),
            "QADIS".to_owned(),
            "QAIDS".to_owned(),
            "RABAT".to_owned(),
            "RABBI".to_owned(),
            "SAAGS".to_owned(),
            "SABAL".to_owned(),
            "TAALS".to_owned(),
            "TAATA".to_owned(),
            "UDALS".to_owned(),
            "UDDER".to_owned(),
            "VACUA".to_owned(),
            "VADED".to_owned(),
            "WAACS".to_owned(),
            "WACKE".to_owned(),
            "XEBEC".to_owned(),
            "XENIA".to_owned(),
            "YAARS".to_owned(),
            "YABAS".to_owned(),
            "ZABRA".to_owned(),
            "ZACKS".to_owned(),
        ]
    }

    #[test]
    fn test_create_word_index_five() {
        let word_index = WordIndexFive::new(create_word_list());

        // for (k, v) in word_index.letter_location_map.iter() {
        //     println!("{:?} : {:?}", k, v);
        // }
    }

    #[test]
    fn test_evaluate_query() {
        let word_index = WordIndexFive::new(create_word_list());

        let query = QueryNode::Op(Box::new(QueryOp {
            kind: QueryOpKind::And,
            left: QueryNode::Location(LetterLocation {
                letter: Letter::A,
                slot: LetterSlot::Two,
            }),
            right: QueryNode::Op(Box::new(QueryOp {
                kind: QueryOpKind::Or,
                left: QueryNode::Location(LetterLocation {
                    letter: Letter::D,
                    slot: LetterSlot::Four,
                }),
                right: QueryNode::Location(LetterLocation {
                    letter: Letter::D,
                    slot: LetterSlot::Three,
                }),
            }))
        }));
        let matching_words = word_index.evaluate_query(query);

        println!("{:?}", matching_words);
    }

    #[test]
    fn try_quordle() {
        let word_list = create_word_list_from_file("/Users/dwadeson/games_workspace/word_resources/scrabble_words_2019_5.txt").unwrap();
        let word_index = WordIndexFive::new(word_list);
        let query = AndNode(
            FixedNode('s', 1),
            AndNode(
                FixedNode('a', 3),
                AndNode(
                    FixedNode('t', 5),
                    OrNode(
                        FixedNode('h', 2),
                        FixedNode('h', 4),    
                    )
                )
            )
        );

        let matching_words = word_index.evaluate_query(query);

        println!("{:?}", matching_words);
    }
}