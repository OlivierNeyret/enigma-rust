use std::collections::HashMap;

pub enum FixedReflectorType {
    B,
    C,
}

pub struct Reflector {
    wiring: HashMap<char, char>,
}

// Here it is a fixed reflector
impl Reflector {
    pub fn new(reflector_type: FixedReflectorType) -> Reflector {
        Reflector {
            wiring: match reflector_type {
                FixedReflectorType::B => {
                    [('A','Y'), ('B','R'), ('C','U'), ('D','H'), ('E','Q'), ('F','S'), ('G','L') ,('H','D'), ('I','P'), ('J','X'), ('K','N'), ('L','G'), ('M','O'), ('N','K'), ('O','M'), ('P','I'), ('Q','E'), ('R','B'), ('S','F'), ('T','Z'), ('U','C'), ('V','W'), ('W','V'), ('X','J'), ('Y','A'), ('Z','T')]
                    .iter().cloned().collect()
                }
                FixedReflectorType::C => {
                    [('A','F'), ('B','V'), ('C','P'), ('D','J'), ('E','I'), ('F','A'), ('G','O') ,('H','Y'), ('I','E'), ('J','D'), ('K','R'), ('L','Z'), ('M','X'), ('N','W'), ('O','G'), ('P','C'), ('Q','T'), ('R','K'), ('S','U'), ('T','Q'), ('U','S'), ('V','B'), ('W','N'), ('X','M'), ('Y','H'), ('Z','L')]
                    .iter().cloned().collect()
                }
            },
        }
    }

    pub fn forward(&self, char_in: char) -> Result<char, String> {
        match self.wiring.get(&char_in) {
            Some(&v) => Ok(v),
            None => Err(String::from("Reflector error : key not found.")), // TODO: find a way to add char_in to error message
        }
    }
}