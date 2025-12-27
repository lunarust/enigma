use serde::{Deserialize, Serialize};

const ROTORS: [&str; 5] = ["BDFHJLCPRTXVZNYEIWGAKMUSQO", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];
const ROMAN: [&str; 5] = ["I", "II", "III", "IV", "V"];

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Ciphertext {
    pub rotor: Vec<i32>,
    pub plain: String,
    pub cryptic: String,
    pub reflector: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CipherRotor {
    pub id: i32,
    pub name: String,
    pub definition: String,
}

pub fn get_rotor(n: i32) -> CipherRotor {
    //n -= 1;
    CipherRotor{
        id: n,
        name: ROMAN[n as usize].to_string(),
        definition: ROTORS[n as usize].to_string() }
}
pub fn rotor_setup () -> Vec<CipherRotor> {
    let mut rt: Vec<CipherRotor> = vec![];
    for n in 0..5 {
        rt.push(CipherRotor {
            id: n,
            name: ROMAN[n as usize].to_string(),
            definition: ROTORS[n as usize].to_string() });
    }
    rt
}
impl Ciphertext {
    pub fn of(ciphertext: Ciphertext) -> Ciphertext {
        Ciphertext {
            rotor: ciphertext.rotor,
            plain: ciphertext.plain,
            cryptic: ciphertext.cryptic,
            reflector: ciphertext.reflector,
        }
    }
}
