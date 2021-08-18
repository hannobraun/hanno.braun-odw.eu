use std::net::Ipv6Addr;

use axum::{
    handler::get, response::IntoResponse, route, routing::RoutingDsl as _,
};

#[tokio::main]
async fn main() {
    let app = route("/", get(hello_world));
    axum::Server::bind(&(Ipv6Addr::UNSPECIFIED, 8080).into())
        .serve(app.into_make_service())
        .await
        // TASK: Improve error handling.
        .unwrap()
}

async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}
