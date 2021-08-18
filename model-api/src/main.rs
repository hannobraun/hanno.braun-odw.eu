use std::net::{Ipv6Addr, SocketAddr};

use axum::{
    handler::get, response::IntoResponse, route, routing::RoutingDsl as _,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = route("/", get(hello_world));
    axum::Server::bind(&SocketAddr::from((Ipv6Addr::UNSPECIFIED, 8080)))
        .serve(app.into_make_service())
        .await
        // TASK: Improve error handling.
        .unwrap()
}

async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}
