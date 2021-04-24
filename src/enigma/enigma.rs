use std::collections::HashMap;

use super::plugboard::PlugBoard;
use super::reflector::*;
use super::rotor::*;

pub struct Enigma {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,

    reflector: Reflector,

    plug_board: PlugBoard,
}

impl Enigma {
    pub fn new(reflector_type: FixedReflectorType, rotor_name: &[RotorName; 3], rotor_position: &[usize; 3], rings_settings: &[usize; 3], plug_board_wiring: &HashMap<char, char>) -> Enigma {
        Enigma {
            left_rotor: Rotor::new(rotor_name[0], rotor_position[0], rings_settings[0]),
            middle_rotor: Rotor::new(rotor_name[1], rotor_position[1], rings_settings[1]),
            right_rotor: Rotor::new(rotor_name[2], rotor_position[2], rings_settings[2]),
            reflector: Reflector::new(reflector_type),
            plug_board: PlugBoard::new(plug_board_wiring),
        }
    }

    pub fn encrypt(&mut self, str_in: &String) -> Result<String, String> {
        let mut str_encrypted = String::new();
        for mut c in str_in.chars() {
            self.rotate();

            c = self.plug_board.forward(c)?;

            c = self.right_rotor.forward(c);
            c = self.middle_rotor.forward(c);
            c = self.left_rotor.forward(c);

            c = self.reflector.forward(c)?;

            c = self.left_rotor.backward(c);
            c = self.middle_rotor.backward(c);
            c = self.right_rotor.backward(c);

            c = self.plug_board.forward(c)?;

            str_encrypted.push(c);
        }
        Ok(str_encrypted)
    }

    fn rotate(&mut self) {
        if self.middle_rotor.is_at_notch() {
            self.middle_rotor.turn_over();
            self.left_rotor.turn_over();
        }
        else if self.right_rotor.is_at_notch() {
            self.middle_rotor.turn_over();
        }

        self.right_rotor.turn_over();
    }
}