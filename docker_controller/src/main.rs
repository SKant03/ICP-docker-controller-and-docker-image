mod docker;
mod types;

use warp::Filter;
use crate::docker::{start_instance, stop_instance};
use crate::types::{SessionRequest, SessionResponse};

#[tokio::main]
async fn main() {
    let start = warp::post()
        .and(warp::path("start"))
        .and(warp::body::json())
        .and_then(start_instance);

    let stop = warp::post()
        .and(warp::path("stop"))
        .and(warp::body::json())
        .and_then(stop_instance);

    let routes = start.or(stop);

    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
