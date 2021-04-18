mod enigma;

use enigma::*;
use std::collections::HashMap;

fn main() {
    println!("Welcome in enigma-rust!");
    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let enigma = Enigma::new(FixedReflectorType::B, &plug_board_wiring);
    let plain_text = String::from("THISISAPLAINTEXT");
    println!("{}", plain_text);
    let encrypted_text = enigma.encrypt(&plain_text).unwrap();
    println!("{}", encrypted_text);
}
