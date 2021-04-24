mod enigma;

use enigma::*;
use std::collections::HashMap;

fn main() {
    println!("Welcome in enigma-rust!");

    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let rotors_name = [RotorName::III, RotorName::I, RotorName::IV];
    let rotors_position = [14, 23, 6];
    let rings_setting = [8, 2, 18];
    let mut enigma = Enigma::new(FixedReflectorType::B, &rotors_name, &rotors_position, &rings_setting, &plug_board_wiring);

    let plain_text = String::from("THISISAPLAINTEXT");
    println!("{}", plain_text);

    let encrypted_text = enigma.encrypt(&plain_text).unwrap();
    println!("{}", encrypted_text);
}
