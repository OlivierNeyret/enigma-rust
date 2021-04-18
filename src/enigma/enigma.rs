use std::collections::HashMap;

use super::plugboard::PlugBoard;
use super::reflector::*;
use super::rotor::Rotor;

pub struct Enigma {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,

    reflector: Reflector,

    plug_board: PlugBoard,
}

impl Enigma {
    pub fn new(reflector_type: FixedReflectorType, plug_board_wiring: &HashMap<char, char>) -> Enigma {
        Enigma {
            left_rotor: Rotor::new(),
            middle_rotor: Rotor::new(),
            right_rotor: Rotor::new(),
            reflector: Reflector::new(reflector_type),
            plug_board: PlugBoard::new(plug_board_wiring),
        }
    }

    pub fn encrypt(&self, str_in: &String) -> Result<String, &str> {
        let mut str_encrypted = String::new();
        for mut c in str_in.chars() {
            self.rotate();

            c = self.plug_board.forward(c)?;

            c = self.right_rotor.forward(c);
            c = self.middle_rotor.forward(c);
            c = self.left_rotor.forward(c);

            c = self.reflector.forward(c)?;

            c = self.left_rotor.forward(c);
            c = self.middle_rotor.forward(c);
            c = self.right_rotor.forward(c);

            c = self.plug_board.forward(c)?;

            str_encrypted.push(c);
        }
        Ok(str_encrypted)
    }

    fn rotate(&self) {

    }
}