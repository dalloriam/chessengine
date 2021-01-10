mod app;

#[tokio::main]
async fn main() {
    app::serve().await;
}
