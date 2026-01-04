use warp::{reply::json, Reply};

use common::*;
use crate::punch;

pub async fn handle_call(body: Ciphertext) -> Result<impl Reply, warp::Rejection>  {

    let t = body.plain.to_lowercase().clone().replace(|c: char| !c.is_alphabetic(), "");

    let (output, debug_logs_list) =
        punch::decrypt(body.rotor.clone(), body.reflector.clone(), t, body.start_position,
        body.plugboard.to_lowercase()
    ).await;

    Ok(json(
        &Response {
            plain: body.plain,
            cryptic: output,
            debug_logs: debug_logs_list
        }
    ))
}
pub async fn handle_decrypt_call(body: Ciphertext) -> Result<impl Reply, warp::Rejection>  {
    let t = body.cryptic.to_lowercase().clone().replace(|c: char| !c.is_alphabetic(), "");

    let (output, debug_logs_list) =
        punch::decrypt(body.rotor.clone(), body.reflector.clone(), t, body.start_position,
        body.plugboard.to_lowercase()).await;

    Ok(json(
        &Response {
            plain: output,
            cryptic: body.cryptic,
            debug_logs: debug_logs_list
        }
    ))
}
