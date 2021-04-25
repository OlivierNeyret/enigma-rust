mod plugboard;
mod reflector;
mod rotor;

use std::collections::HashMap;

use plugboard::PlugBoard;
use reflector::*;
use rotor::*;

pub struct RotorConfig {
    name: RotorName,
    position: usize,
    ring_setting: usize,
}

pub struct Enigma {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,

    reflector: Reflector,

    plug_board: PlugBoard,
}

impl Enigma {
    pub fn new(reflector_type: FixedReflectorType, rotor_left: &RotorConfig, rotor_center: &RotorConfig, rotor_right: &RotorConfig, plug_board_wiring: &HashMap<char, char>) -> Enigma {
        Enigma {
            left_rotor: Rotor::new(rotor_left.name, rotor_left.position, rotor_left.ring_setting),
            middle_rotor: Rotor::new(rotor_center.name, rotor_center.position, rotor_center.ring_setting),
            right_rotor: Rotor::new(rotor_right.name, rotor_right.position, rotor_right.ring_setting),
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