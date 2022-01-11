use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    value: Option<u8>,
    is_final: bool,
    child_nodes: HashMap<u8, TrieNode>,
}

impl TrieNode {
    pub fn new(ch: u8, is_final: bool) -> TrieNode {
        TrieNode {
            value: Some(ch),
            is_final,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            value: Option::None,
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }

    pub fn check_value(self, ch: u8) -> bool {
        self.value == Some(ch)
    }

    pub fn insert_value(&mut self, ch: u8, is_final: bool) {
        self.child_nodes.insert(ch, TrieNode::new(ch, is_final));
    }
}

#[derive(Debug, PartialEq)]
pub struct TrieSearchOutcome {
    pub is_word: bool,
    pub has_longer_words: bool,
}

#[derive(Debug)]
pub struct WordTrie {
    root_node: TrieNode,
}

impl WordTrie {
    pub fn new() -> WordTrie {
        WordTrie {
            root_node: TrieNode::new_root(),
        }
    }

    pub fn from_words(words: &[&str]) -> WordTrie {
        let mut trie = WordTrie::new();

        for word in words {
            trie.insert(word);
        }

        trie
    }

    // Insert a string
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root_node;
        let char_list: &[u8] = word.as_bytes();
        let mut last_match = 0;

        for letter_counter in 0..char_list.len() {
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                last_match = letter_counter;
                break;
            }
            last_match = letter_counter + 1;
        }

        if last_match == char_list.len() {
            current_node.is_final = true;
        } else {
            for new_counter in last_match..char_list.len() {
                // println!(
                //     "Inserting {} into {}",
                //     char_list[new_counter],
                //     current_node.value.unwrap_or_default()
                // );
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            current_node.is_final = true;
        }
    }

    // Find a string
    pub fn find(&mut self, char_list: &[u8]) -> TrieSearchOutcome {
        let mut current_node = &mut self.root_node;
        let mut all_chars_processed = true;

        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                all_chars_processed = false;
                break;
            } else {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[counter])
                    .unwrap();
            }
        }

        if all_chars_processed {
            TrieSearchOutcome {
                is_word: current_node.is_final,
                has_longer_words: !current_node.child_nodes.is_empty(),
            }
        } else {
            TrieSearchOutcome {
                is_word: false,
                has_longer_words: false
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let words = vec![
            "apple",
            "app",
            "a",
            "zeta",
            "apply",
            "apt",
        ];

        let mut trie = WordTrie::from_words(&words);

        assert_eq!(
            trie.find("a".as_bytes()),
            TrieSearchOutcome {
                is_word: true,
                has_longer_words: true,
            },
        );

        assert_eq!(
            trie.find("app".as_bytes()),
            TrieSearchOutcome {
                is_word: true,
                has_longer_words: true,
            },
        );

        assert_eq!(
            trie.find("apps".as_bytes()),
            TrieSearchOutcome {
                is_word: false,
                has_longer_words: false,
            },
        );

        assert_eq!(
            trie.find("ba".as_bytes()),
            TrieSearchOutcome {
                is_word: false,
                has_longer_words: false,
            },
        );

        assert_eq!(
            trie.find("zeta".as_bytes()),
            TrieSearchOutcome {
                is_word: true,
                has_longer_words: false,
            },
        );

        assert_eq!(
            trie.find("ap".as_bytes()),
            TrieSearchOutcome {
                is_word: false,
                has_longer_words: true,
            },
        )
    }
}