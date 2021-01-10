use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use super::ServerRC;

#[derive(Deserialize, Serialize)]
struct MessageResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct MovePayload {
    src: String,
    dst: String,
}

#[derive(Deserialize, Serialize)]
struct PositionResponse {
    position_fen: String,
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn hello(_srv: ServerRC) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&MessageResponse {
        message: String::from("hello"),
    }))
}

pub async fn do_move(item: MovePayload, srv: ServerRC) -> Result<impl warp::Reply, Infallible> {
    Ok(match srv.do_move(&item.src, &item.dst) {
        Ok(position_fen) => warp::reply::json(&PositionResponse { position_fen }),
        Err(e) => {
            let x = e.to_string();
            println!("Error: {}", x);
            warp::reply::json(&ErrorResponse { error: x })
        }
    })
}

pub async fn get_position(srv: ServerRC) -> Result<impl warp::Reply, Infallible> {
    Ok(match srv.get_position() {
        Ok(position_fen) => warp::reply::json(&PositionResponse { position_fen }),
        Err(e) => warp::reply::json(&ErrorResponse {
            error: e.to_string(),
        }),
    })
}
