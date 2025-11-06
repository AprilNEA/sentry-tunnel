use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Create a standalone Sentry tunnel service and nest it under a path
    let sentry_service = sentry_tunnel::create_sentry_tunnel_service(
        sentry_tunnel::SentryTunnelConfig::new(
            "o123456.ingest.sentry.io",
            vec!["123456".to_string(), "789012".to_string()],
        ),
    );

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/sentry", sentry_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    println!("Sentry tunnel endpoint: http://127.0.0.1:3000/sentry/tunnel");

    axum::serve(listener, app).await.unwrap();
}
