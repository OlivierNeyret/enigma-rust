use std::collections::HashMap;

use super::reflector::*;
use super::plugboard::PlugBoard;

pub struct Enigma {
    // left_rotor: Rotor,
    // middle_rotor: Rotor,
    // right_rotor: Rotor,

    reflector: Reflector,

    plug_board: PlugBoard,
}

impl Enigma {
    pub fn new(plug_board_wiring: &HashMap<char, char>) -> Enigma {
        Enigma {
            reflector: Reflector::new(FixedReflectorType::B),
            plug_board: PlugBoard::new(plug_board_wiring),
        }
    }
}