use axum::routing::get;
use sentry_tunnel::{SentryTunnelBuilder, SentryTunnelExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sentry Tunnel API",
        version = "0.1.0",
        description = "API for tunneling Sentry requests through your own domain"
    ),
    tags(
        (name = "sentry", description = "Sentry tunnel endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Create OpenApiRouter with Sentry tunnel support
    // The sentry_tunnel method works with OpenApiRouter just like with regular Router
    let (router, _api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(|| async { "Hello, World!" }))
        .sentry_tunnel(
            SentryTunnelBuilder::new("o123456.ingest.sentry.io")
                .allow_project_ids(["123456", "789012"])
                .path("/tunnel")
                .timeout_secs(30)
                .build(),
        )
        .split_for_parts();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    println!("Sentry tunnel endpoint: http://127.0.0.1:3000/tunnel");
    println!("OpenAPI documentation is available via the OpenApiRouter");

    axum::serve(listener, router).await.unwrap();
}
