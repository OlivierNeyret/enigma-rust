mod enigma;

use enigma::Enigma;
use std::collections::HashMap;

fn main() {
    println!("Welcome in enigma-rust!");
    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let enigma = Enigma::new(&plug_board_wiring);
}
