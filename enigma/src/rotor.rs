#[derive(Copy, Clone)]
pub enum RotorName {
    I,
    II,
    III,
    IV,
    V,
    // VI,
    // VII,
    // VIII,
}

pub struct Rotor {
    // Here we can't use HashMap because we need to keep the order
    wiring: Vec<char>,
    reversed_wiring: Vec<char>,

    rotor_position: usize,
    notch_position: usize,
    ring_setting: usize,
}

impl Rotor {
    pub fn new(name: RotorName, position: usize, ring_setting: usize) -> Rotor {
        match name {
            RotorName::I => {
                let forward_wiring = vec!['E','K','M','F','L','G','D','Q','V','Z','N','T','O','W','Y','H','X','U','S','P','A','I','B','R','C','J',];
                let reversed_wiring = reverse_wiring(&forward_wiring);
                Rotor {
                    wiring: forward_wiring,
                    reversed_wiring: reversed_wiring,
                    rotor_position: position,
                    notch_position: 16,
                    ring_setting: ring_setting,
                }
            },
            RotorName::II => {
                let forward_wiring = vec!['A','J','D','K','S','I','R','U','X','B','L','H','W','T','M','C','Q','G','Z','N','P','Y','F','V','O','E',];
                let reversed_wiring = reverse_wiring(&forward_wiring);
                Rotor {
                    wiring: forward_wiring,
                    reversed_wiring: reversed_wiring,
                    rotor_position: position,
                    notch_position: 4,
                    ring_setting: ring_setting,
                }
            },
            RotorName::III => {
                let forward_wiring = vec!['B','D','F','H','J','L','C','P','R','T','X','V','Z','N','Y','E','I','W','G','A','K','M','U','S','Q','O',];
                let reversed_wiring = reverse_wiring(&forward_wiring);
                Rotor {
                    wiring: forward_wiring,
                    reversed_wiring: reversed_wiring,
                    rotor_position: position,
                    notch_position: 21,
                    ring_setting: ring_setting,
                }
            },
            RotorName::IV => {
                let forward_wiring = vec!['E','S','O','V','P','Z','J','A','Y','Q','U','I','R','H','X','L','N','F','T','G','K','D','C','M','W','B',];
                let reversed_wiring = reverse_wiring(&forward_wiring);
                Rotor {
                    wiring: forward_wiring,
                    reversed_wiring: reversed_wiring,
                    rotor_position: position,
                    notch_position: 9,
                    ring_setting: ring_setting,
                }
            },
            RotorName::V => {
                let forward_wiring = vec!['V','Z','B','R','G','I','T','Y','U','P','S','D','N','H','L','X','A','W','M','J','Q','O','F','E','C','K'];
                let reversed_wiring = reverse_wiring(&forward_wiring);
                Rotor {
                    wiring: forward_wiring,
                    reversed_wiring: reversed_wiring,
                    rotor_position: position,
                    notch_position: 25,
                    ring_setting: ring_setting,
                }
            },
            // TODO: find a way to implement rotor VI, VII and VII with two notches
        }
    }

    pub fn forward(&self, char_in: char) -> char {
        encipher((char_in as usize) - 65, self.rotor_position, self.ring_setting, &self.wiring)
    }

    pub fn backward(&self, char_in: char) -> char {
        encipher((char_in as usize) - 65, self.rotor_position, self.ring_setting, &self.reversed_wiring)

    }

    pub fn is_at_notch(&self) -> bool {
        self.notch_position == self.rotor_position
    }

    pub fn turn_over(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26;
    }
}

fn reverse_wiring(forward: &Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = Vec::with_capacity(forward.len());
    result.resize(forward.len(), 'a');
    for i in 0..forward.len() {
        let c = forward[i];
        result.insert(c as usize - 65, (i + 65) as u8 as char);
    }
    result
}

fn encipher(k: usize, pos: usize, ring: usize, wiring: &Vec<char>) -> char {
    let shift: i32 = pos as i32 - ring as i32;
    let index: usize = (k as i32 + shift + 26) as usize % 26;
    let a: i32 = (wiring[index] as i32) - 65;
    let b = (a - shift + 26) % 26;
    (b + 65) as u8 as char
}
