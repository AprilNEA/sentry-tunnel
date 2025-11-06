use axum::{routing::get, Router};
use sentry_tunnel::{SentryTunnelBuilder, SentryTunnelExt};

#[tokio::main]
async fn main() {
    // Using extension trait to add Sentry tunnel to existing Router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .sentry_tunnel(
            SentryTunnelBuilder::new("o123456.ingest.sentry.io")
                .allow_project_ids(["123456", "789012"])
                .path("/tunnel")
                .timeout_secs(30)
                .build(),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    println!("Sentry tunnel endpoint: http://127.0.0.1:3000/tunnel");

    axum::serve(listener, app).await.unwrap();
}
