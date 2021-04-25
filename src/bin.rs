// mod enigma;

use enigma::rotor::RotorName;
use enigma::*;
use std::collections::HashMap;

fn main() {
    println!("Welcome in enigma-rust!");

    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let rotor_left = RotorConfig {
        name: RotorName::III,
        position: 14, 
        ring_setting: 8};
    let rotor_middle = RotorConfig {
        name: RotorName::I,
        position: 23, 
        ring_setting: 2};
    let rotor_center = RotorConfig {
        name: RotorName::IV,
        position: 6, 
        ring_setting: 18};

    let mut enigma = Enigma::new(FixedReflectorType::B, &rotor_left, &rotor_middle, &rotor_center, &plug_board_wiring);

    let plain_text = String::from("THISISAPLAINTEXT");
    println!("{}", plain_text);

    let encrypted_text = enigma.encrypt(&plain_text).unwrap();
    println!("{}", encrypted_text);
}
