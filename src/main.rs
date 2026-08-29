use rrcounter::{Config, build_app};

#[tokio::main]
async fn main() {
    let config = Config::default();
    let router = build_app(&config).await;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", &config.server_port))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
