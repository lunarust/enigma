use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Ciphertext {
    pub rotor: i32,
    pub plain: String,
    pub cryptic: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CiphertextRequest {
    pub rotor: i32,
    pub plain: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CiphertextResponse {
    pub rotor: i32,
    pub plain: String,
    pub cryptic: String,
}

impl CiphertextResponse {
    pub fn of(ciphertext: Ciphertext) -> CiphertextResponse {
        CiphertextResponse {
            rotor: ciphertext.rotor,
            plain: ciphertext.plain,
            cryptic: ciphertext.cryptic,
        }
    }
}
