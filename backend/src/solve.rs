use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

use common::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Solve {
    rotor: Vec<i32>,
    plain: String,
    cryptic: String,
    reflector: String,
}

pub async fn handle_call(body: Ciphertext) ->   Result<impl Reply, warp::Rejection>  {
    println!("Solving {:?} ", body);

    let my_rotors: Vec<CipherRotor> = vec![
        get_rotor(body.rotor[0]),
        get_rotor(body.rotor[1]),
        get_rotor(body.rotor[2])
    ];

    println!("For BODY: {:?} Fetching my rotors: {:?} ", body, my_rotors);

    Ok(json(&Ciphertext {
        rotor: [4,5,2].to_vec(),
        plain: body.plain,
        cryptic: "wepojfeif".to_string(),
        reflector: "".to_string()
    }))
}
pub async fn handle_rejection(value: String) -> Result<impl Reply, warp::Rejection> {
    println!("Rejecting this {}", value);
    Ok(warp::reply::with_status("OK", StatusCode::OK))
}
