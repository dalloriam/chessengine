mod filters;
mod game_server;
mod handlers;

use game_server::GameServer;

use std::sync::Arc;

type ServerRC = Arc<GameServer>;

pub async fn serve() {
    let server = Arc::new(GameServer::new());
    warp::serve(filters::all(server))
        .run(([127, 0, 0, 1], 3030))
        .await
}
