use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

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
    println!("Encrypting {:?} ", body);

    //let t = body.plain.clone().replace(|c: char| !c.is_ascii(), "");
    let t = body.plain.to_lowercase().clone().replace(|c: char| !c.is_alphabetic(), "");

    //println!("Clean text: {}", t);

    let (output, debug_logs_list) = punch::decrypt(body.rotor.clone(), body.reflector.clone(), t, body.start_position).await;

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

    let (output, debug_logs_list) = punch::decrypt(body.rotor.clone(), body.reflector.clone(), t, body.start_position).await;

    Ok(json(
        &Response {
            plain: output,
            cryptic: body.cryptic,
            debug_logs: debug_logs_list
        }
    ))
}
pub async fn handle_rejection(value: String) -> Result<impl Reply, warp::Rejection> {
    println!("Rejecting this {}", value);
    Ok(warp::reply::with_status("OK", StatusCode::OK))
}
