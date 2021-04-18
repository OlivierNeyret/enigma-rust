// mod reflector;

use super::reflector::*;

pub struct Enigma {
    // left_rotor: Rotor,
    // middle_rotor: Rotor,
    // right_rotor: Rotor,

    reflector: Reflector,
}

impl Enigma {
    pub fn new() -> Enigma {
        Enigma {
            reflector: Reflector::new(FixedReflectorType::B),
        }
    }
}