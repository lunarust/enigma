#[macro_use]
extern crate lazy_static;
use common::*;

use warp::{
    http::{header, Method},
    Filter, Rejection,
};
mod solve;

#[tokio::main]
async fn main() {

    println!("Good day ▼(´ᴥ`)▼ ");

    let scrumble = warp::path("scrumble");
    let scrumble_routes = scrumble
           .and(warp::post())
           .and(warp::body::json())
           .and_then(solve::handle_call);

    let hello = warp::path!("hello" / String) // 3.
        .map(|name| format!("Hello, {}!", name)); // 4.


    let route = warp::method()
        .map(|method| {
            format!("You sent a {} request!", method)
    });


    let routes = scrumble_routes
    .or(hello)
    .with(
        warp::cors()
            .allow_origin("http://localhost")
            .allow_origin("http://aetes")
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
                Method::HEAD,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_headers(vec!["allow_origin", "allow_any_origin", "Access-Control-Allow-Origin",
                "Referer", "Control-Request-Headers", "Content-Type"])
            //        .allow_headers(vec!["Access-Control-Allow-Origin", "Origin", "Accept", "X-Requested-With", "Content-Type"])
            .max_age(300)
            .allow_any_origin(),
    );
    warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
}
