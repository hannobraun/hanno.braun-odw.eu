use std::net::{Ipv6Addr, SocketAddr};

use axum::{
    handler::get, response::IntoResponse, route, routing::RoutingDsl as _,
};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt().init();

    let app = route("/", get(hello_world)).layer(
        TraceLayer::new_for_http()
            // TASK: Log request path.
            .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 8080));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        // TASK: Improve error handling.
        .unwrap()
}

async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}
