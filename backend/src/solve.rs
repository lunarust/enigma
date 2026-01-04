use serde::{Deserialize, Serialize};
use warp::{reply::json, Reply};

use common::*;
use crate::punch;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Solve {
    rotor: Vec<i32>,
    plain: String,
    cryptic: String,
    reflector: String,
}

pub async fn handle_call(body: Ciphertext) -> Result<impl Reply, warp::Rejection>  {
    //println!("Encrypting {:?} ", body);
    //let t = body.plain.clone();
    //let t = body.plain.clone().replace(|c: char| !c.is_ascii(), "");
    let t = body.plain.to_lowercase().clone().replace(|c: char| !c.is_alphabetic(), "");
    //t = plugboard_swich(t, body.plugboard.to_lowercase());

    //println!("{:?}", t);


    let (output, debug_logs_list) =
        punch::decrypt(body.rotor.clone(), body.reflector.clone(), t, body.start_position,
        body.plugboard.to_lowercase()
    ).await;

    //let res = plugboard_swich(output, body.plugboard.to_lowercase());

    Ok(json(
        &Response {
            plain: body.plain,
            cryptic: output,
            debug_logs: debug_logs_list
        }
    ))
}
pub async fn handle_decrypt_call(body: Ciphertext) -> Result<impl Reply, warp::Rejection>  {
    println!("Decrypting {:?} ", body);

    //let t = body.plain.clone().replace(|c: char| !c.is_ascii(), "");
    let t = body.cryptic.to_lowercase().clone().replace(|c: char| !c.is_alphabetic(), "");
    //t = plugboard_swich(t, body.plugboard.to_lowercase());


    let (output, debug_logs_list) =
        punch::decrypt(body.rotor.clone(), body.reflector.clone(), t, body.start_position,
        body.plugboard.to_lowercase()).await;

    //let res = plugboard_swich(output, body.plugboard.to_lowercase());

    Ok(json(
        &Response {
            plain: output,
            cryptic: body.cryptic,
            debug_logs: debug_logs_list
        }
    ))
}
