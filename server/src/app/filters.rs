use serde::de::DeserializeOwned;
use warp::Filter;

use super::{handlers, ServerRC};

const HELLO_ROUTE_PATH: &str = "hello";
const MOVE_ROUTE_PATH: &str = "move";
const POSITION_ROUTE_PATH: &str = "position";

fn with_server(
    srv: ServerRC,
) -> impl Filter<Extract = (ServerRC,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || srv.clone())
}

fn json_body<T: Send + DeserializeOwned>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn all(
    srv: ServerRC,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    hello(srv.clone())
        .or(get_position(srv.clone()))
        .or(do_move(srv))
        .with(cors)
}

pub fn hello(
    srv: ServerRC,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path(HELLO_ROUTE_PATH))
        .and(with_server(srv))
        .and_then(handlers::hello)
}

pub fn do_move(
    srv: ServerRC,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path(MOVE_ROUTE_PATH))
        .and(json_body::<handlers::MovePayload>())
        .and(with_server(srv))
        .and_then(handlers::do_move)
}

pub fn get_position(
    srv: ServerRC,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path(POSITION_ROUTE_PATH))
        .and(with_server(srv))
        .and_then(handlers::get_position)
}
