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
    println!("Solving {:?} ", body);

    let (output, debug_logs_list) = punch::decrypt(body.rotor.clone(), body.reflector.clone(), body.plain.clone()).await;

    Ok(json(
        &Response {
            plain: body.plain,
            cryptic: output,
            debug_logs: debug_logs_list
        }
    ))
}
pub async fn handle_rejection(value: String) -> Result<impl Reply, warp::Rejection> {
    println!("Rejecting this {}", value);
    Ok(warp::reply::with_status("OK", StatusCode::OK))
}
