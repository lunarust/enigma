use serde::{Deserialize, Serialize};
use serde_json;

const REFLECTORS_DATA: &str = r#"[{"id":1,"name": "Beta", "definition": "LEYJVCNIXWPBQMDRTAKZGFUHOS", "model": "M4 R2"},
{"id":2,"name": "Gamma", "definition": "FSOKANUERHMBTIYCWLQPZXVGJD", "model": "M4 R2"},
{"id":3,"name": "Reflector A", "definition": "EJMZALYXVBWFCRQUONTSPIKHGD", "model": ""},
{"id":4,"name": "Reflector B", "definition": "YRUHQSLDPXNGOKMIEBFZCWVJAT", "model": "M3"},
{"id":5,"name": "Reflector C", "definition": "FVPJIAOYEDRZXWGCTKUQSBNMHL", "model": "M3"},
{"id":6,"name": "Reflector B Dünn", "definition": "ENKQAUYWJICOPBLMDXZVFTHRGS", "model": "M4 R1 (M3 + Dünn)"},
{"id":7,"name": "Reflector C Dünn", "definition": "RDOBJNTKVEHMLFCWZAXGYIPSUQ", "model": "M4 R1 (M3 + Dünn)"}]
"#;

const ROTORS_DATA: &str =
r#"[{"id":0,"name":"I", "definition": "EKMFLGDQVZNTOWYHXUSPAIBRCJ", "model": "Enigma I", "notch": ["Q"] },
{"id":1,"name":"II", "definition": "AJDKSIRUXBLHWTMCQGZNPYFVOE", "model": "Enigma I", "notch": ["E"] },
{"id":2,"name":"III", "definition": "BDFHJLCPRTXVZNYEIWGAKMUSQO", "model": "Enigma I", "notch": ["V"] },
{"id":3,"name":"IV", "definition": "ESOVPZJAYQUIRHXLNFTGKDCMWB", "model": "M3 Army", "notch": ["J"] },
{"id":4,"name":"V", "definition": "VZBRGITYUPSDNHLXAWMJQOFECK", "model": "M3 Army", "notch": ["Z"] },
{"id":5,"name":"VI", "definition": "JPGVOUMFYQBENHZRDKASXLICTW", "model": "M3 & M4 Naval", "notch": ["Z","M"] },
{"id":6,"name":"VII", "definition": "NZJHGRCXMYSWBOUFAIVLPEKQDT", "model": "M3 & M4 Naval", "notch": ["Z","M"] },
{"id":7,"name":"VIII", "definition": "FKQHTLXOCBJSPDZRAMEWNIUYGV", "model": "M3 & M4 Naval", "notch": ["Z","M"] },
{"id":8,"name":"I", "definition": "JGDQOXUSCAMIFRVTPNEWKBLZYH", "model": "German Railway (Rocket)", "notch": ["Z"]},
{"id":9,"name":"II", "definition": "NTZPSFBOKMWRCJDIVLAEYUXHGQ", "model": "German Railway (Rocket)", "notch": ["Z"]},
{"id":10,"name":"III", "definition": "JVIUBHTCDYAKEQZPOSGXNRMWFL", "model": "German Railway (Rocket)", "notch": ["Z"]},
{"id":11,"name":"UKW", "definition": "QYHOGNECVPUZTFDJAXWMKISRBL", "model": "German Railway (Rocket)", "notch": ["Z"]},
{"id":12,"name":"ETW", "definition": "QWERTZUIOASDFGHJKPYXCVBNML", "model": "German Railway (Rocket)", "notch": ["Z"]}
]
"#;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Reflector {
    pub id: i32,
    pub name: String,
    pub definition: String,
    pub model: String,
}
pub fn reflector_setup () -> Vec<Reflector> {
    let rt: Vec<Reflector> = serde_json::from_str(REFLECTORS_DATA)
        .expect("error while reading or parsing");
    rt
}
impl Default for Reflector {
    fn default() -> Self {
        Self {
            id: 4,
            name: "Reflector B".to_string(),
            definition: "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string(),
            model: "M3".to_string()
        }
   }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CipherRotor {
    pub id: i32,
    pub name: String,
    pub definition: String,
    pub model: String,
    pub notch: Vec<char>,
}

impl Default for CipherRotor {
    fn default() -> Self {
        Self {
            id: 0,
            name: "I".to_string(),
            definition: "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string(),
            model: "Enigma I".to_string(),
            notch: ['Q'].to_vec(),
        }
   }
}

pub fn rotor_setup () -> Vec<CipherRotor> {
    let rt: Vec<CipherRotor> = serde_json::from_str(ROTORS_DATA)
        .expect("error while reading or parsing");
    rt
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Ciphertext {
    pub rotor: Vec<CipherRotor>,
    pub plain: String,
    pub cryptic: String,
    pub reflector: Reflector,
    pub start_position: Vec<String>,
    pub plugboard: String,
}
impl Ciphertext {
    pub fn of(ciphertext: Ciphertext) -> Ciphertext {
        Ciphertext {
            rotor: ciphertext.rotor,
            plain: ciphertext.plain,
            cryptic: ciphertext.cryptic,
            reflector: ciphertext.reflector,
            start_position: ciphertext.start_position,
            plugboard: ciphertext.plugboard,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DebugLogs {
    pub idx: i32,
    pub offset: Vec<char>,
    pub pass: Vec<String>,
}
impl DebugLogs {
    pub fn of (debug_logs: DebugLogs) -> DebugLogs {
        DebugLogs {
            idx: debug_logs.idx,
            offset: debug_logs.offset,
            pass: debug_logs.pass,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Response {
    pub plain: String,
    pub cryptic: String,
    pub debug_logs: Vec<DebugLogs>,
}
impl Response {
    pub fn of (resp: Response) -> Response {
        Response {
            plain: resp.plain,
            cryptic: resp.cryptic,
            debug_logs: resp.debug_logs,
        }
    }
}
