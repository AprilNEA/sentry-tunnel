# Sentry Tunnel

[![Crates.io](https://img.shields.io/crates/v/sentry-tunnel.svg)](https://crates.io/crates/sentry-tunnel)
[![Documentation](https://docs.rs/sentry-tunnel/badge.svg)](https://docs.rs/sentry-tunnel)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/sentry-tunnel.svg)](https://github.com/AprilNEA/sentry-tunnel#license)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg?maxAge=3600)](https://github.com/AprilNEA/sentry-tunnel)
[![Downloads](https://img.shields.io/crates/d/sentry-tunnel.svg)](https://crates.io/crates/sentry-tunnel)

A flexible Sentry tunnel middleware for Axum that helps bypass ad-blockers and content blockers by proxying Sentry error reports through your own domain.

## Features

- **Modular Architecture**: Choose only the features you need
- **Security**: Validates DSN and project IDs to prevent abuse
- **Flexible Integration**: Router extension trait or standalone service
- **Framework Agnostic Core**: Use the core handler with any web framework
- **Type-Safe**: Built with Rust's type system for reliability

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sentry-tunnel = "0.1"
```

### Feature Flags

- **`extension`** (default): Router extension trait for adding tunnel to existing Router
- **`standalone`**: Standalone service creation (depends on `extension`)
- **`service`**: Convenience feature enabling both `extension` and `standalone`

To disable default features and use only the core:

```toml
[dependencies]
sentry-tunnel = { version = "0.1", default-features = false }
```

## Usage

### Method 1: Router Extension Trait (Recommended)

Add the Sentry tunnel directly to your existing Axum router:

```rust
use axum::{routing::get, Router};
use sentry_tunnel::{SentryTunnelBuilder, SentryTunnelExt};

#[tokio::main]
async fn main() {
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

    axum::serve(listener, app).await.unwrap();
}
```

### Method 2: Standalone Service

Create a standalone Sentry tunnel service and nest it under a path:

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
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

    axum::serve(listener, app).await.unwrap();
}
```

### Method 3: Core Handler (Framework Agnostic)

Use the core handler with any web framework:

```rust
use sentry_tunnel::{handle_sentry_tunnel_inner, SentryTunnelConfig};
use std::sync::Arc;

async fn my_handler(body: &[u8]) -> Result<(), sentry_tunnel::SentryTunnelError> {
    let config = Arc::new(SentryTunnelConfig::new(
        "o123456.ingest.sentry.io",
        vec!["123456".to_string()],
    ));

    handle_sentry_tunnel_inner(config, body).await
}
```

## Configuration

### Using Builder Pattern

```rust
use sentry_tunnel::SentryTunnelBuilder;

let config = SentryTunnelBuilder::new("o123456.ingest.sentry.io")
    .allow_project_id("123456")
    .allow_project_id("789012")
    .path("/tunnel")
    .timeout_secs(30)
    .build();
```

### Direct Configuration

```rust
use sentry_tunnel::SentryTunnelConfig;

let config = SentryTunnelConfig::new(
    "o123456.ingest.sentry.io",
    vec!["123456".to_string()],
)
.with_path("/custom-tunnel")
.with_timeout(60);
```

## Client Configuration

Configure your Sentry client to use the tunnel:

### JavaScript/TypeScript

```javascript
Sentry.init({
  dsn: "https://examplePublicKey@o123456.ingest.sentry.io/123456",
  tunnel: "/tunnel", // or "/sentry/tunnel" if using nested service
});
```

### Python

```python
import sentry_sdk

sentry_sdk.init(
    dsn="https://examplePublicKey@o123456.ingest.sentry.io/123456",
    tunnel="https://yourdomain.com/tunnel",
)
```

## Security

The tunnel validates:
- **Hostname**: Ensures requests only go to your configured Sentry host
- **Project IDs**: Only allows whitelisted project IDs
- **DSN Format**: Validates the DSN structure

This prevents:
- Unauthorized use of your tunnel
- Proxying requests to arbitrary domains
- Resource abuse

## Examples

Run the examples:

```bash
# Extension trait example
cargo run --example extension --features extension

# Standalone service example
cargo run --example standalone --features standalone

# Both methods
cargo run --example basic --features service
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
