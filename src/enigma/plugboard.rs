use std::collections::HashMap;

pub struct PlugBoard {
    wiring: HashMap<char, char>,
}

impl PlugBoard {
    pub fn new(connections: &HashMap<char, char>) -> PlugBoard {
        let mut plug_board = PlugBoard {
            wiring: connections.clone()
        };

        if plug_board.wiring.len() < 26 {
            plug_board.add_missing_combinations();
        }

        return plug_board;
    }

    fn add_missing_combinations(&mut self) {
        for c in b'A'..=b'Z' {
            let c_char: char = c as char;
            if !self.wiring.contains_key(&c_char) {
                self.wiring.insert(c_char, c_char);
            }
        }
    }

    pub fn forward(&self, char_in: char) -> Option<char> {
        match self.wiring.get(&char_in) {
            Some(&v) => Some(v),
            None => None,
        }
    }
}