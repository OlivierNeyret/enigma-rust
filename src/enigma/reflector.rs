pub enum FixedReflectorType {
    B,
    C,
}

pub struct Reflector {
    wiring: [char; 26],
}

// Here it is a fixed reflector
impl Reflector {
    pub fn new(reflector_type: FixedReflectorType) -> Reflector {
        Reflector {
            wiring: match reflector_type {
                FixedReflectorType::B => {
                    ['Y','R','U','H','Q','S','L','D','P','X','N','G','O','K','M','I','E','B','F','Z','C','W','V','J','A','T']
                }
                FixedReflectorType::C => {
                    ['F','V','P','J','I','A','O','Y','E','D','R','Z','X','W','G','C','T','K','U','Q','S','B','N','M','H','L']
                }
            },
        }
    }

    pub fn forward(&self, pos: usize) -> u32 {
        self.wiring[pos] as u32
    }
}