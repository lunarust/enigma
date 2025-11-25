use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

use common::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Solve {
    rotor: i32,
    plain: String,
    cryptic: String,
}

pub async fn handle_call(body: CiphertextRequest) ->   Result<impl Reply, warp::Rejection>  {
    println!("Solving {:?} ", body);

    Ok(json(&CiphertextResponse {
        rotor: 4,
        plain: body.plain,
        cryptic: "wepojfeif".to_string() }))
}
pub async fn handle_rejection(value: String) -> Result<impl Reply, warp::Rejection> {
    println!("Rejecting this {}", value);
    Ok(warp::reply::with_status("OK", StatusCode::OK))
}
