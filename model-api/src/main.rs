use std::{
    env,
    net::{Ipv6Addr, SocketAddr},
};

use axum::{
    body::Body, handler::get, http::Request, response::IntoResponse, route,
    routing::RoutingDsl as _,
};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a filter that defaults to INFO, but can be overridden by a user-
    // supplied `RUST_LOG` env variable. Workaround for:
    // https://github.com/tokio-rs/tracing/issues/1466
    let default_level = Level::INFO;
    let mut directives = default_level.to_string();
    if let Ok(env_var) = env::var("RUST_LOG") {
        if !env_var.is_empty() {
            directives.push(',');
            directives.push_str(&env_var);
        }
    }
    let filter = EnvFilter::new(directives);

    tracing_subscriber::fmt::fmt()
        .with_env_filter(filter)
        .init();

    let app = route("/", get(hello_world)).layer(
        TraceLayer::new_for_http()
            // Required to make request parameters visible in log message when
            // logging on INFO level.
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
            .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 8080));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}
