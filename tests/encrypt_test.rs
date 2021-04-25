use std::collections::HashMap;

use enigma::*;

#[test]
fn encrypt_basic() {
    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let rotor_left = RotorConfig::new(RotorName::I, 0, 0);
    let rotor_middle = RotorConfig::new(RotorName::II, 0, 0);
    let rotor_right = RotorConfig::new(RotorName::III, 0, 0);

    let mut enigma = Enigma::new(FixedReflectorType::B, &rotor_left, &rotor_middle, &rotor_right, &plug_board_wiring);

    let plain_text = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let expected = String::from("BJELRQZVJWARXSNBXORSTNCFMEYHCXTGYJFLINHNXSHIUNTHEORXOPLOVFEKAGADSPNPCMHRVZCYECDAZIHVYGPITMSRZKGGHLSRBLHL");

    let encrypted_text = enigma.encrypt(&plain_text).unwrap();
    assert_eq!(encrypted_text, expected);
}

#[test]
fn decrypt_test() {
    let plug_board_wiring: HashMap<char, char> = HashMap::new();
    let rotor_left = RotorConfig::new(RotorName::I, 0, 0);
    let rotor_middle = RotorConfig::new(RotorName::II, 0, 0);
    let rotor_center = RotorConfig::new(RotorName::III, 0, 0);

    let mut enigma = Enigma::new(FixedReflectorType::B, &rotor_left, &rotor_middle, &rotor_center, &plug_board_wiring);
    let original = String::from("THISISATEST");
    let encrypted = enigma.encrypt(&original).unwrap();

    enigma = Enigma::new(FixedReflectorType::B, &rotor_left, &rotor_middle, &rotor_center, &plug_board_wiring);
    let decrypted = enigma.encrypt(&encrypted).unwrap();
    assert_eq!(decrypted, original);

}