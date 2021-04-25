use enigma::*;
use std::collections::HashMap;

fn main() {
    println!("Welcome in enigma-rust!");

    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let rotor_left = RotorConfig::new(RotorName::III, 14, 8);
    let rotor_middle = RotorConfig::new(RotorName::I, 23, 2);
    let rotor_center = RotorConfig::new(RotorName::IV, 6, 18);

    let mut enigma = Enigma::new(FixedReflectorType::B, &rotor_left, &rotor_middle, &rotor_center, &plug_board_wiring);

    let plain_text = String::from("THISISAPLAINTEXT");
    println!("{}", plain_text);

    let encrypted_text = enigma.encrypt(&plain_text).unwrap();
    println!("{}", encrypted_text);
}
