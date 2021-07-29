use std::net::Ipv6Addr;

use warp::Filter as _;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let hello = warp::any()
        .map(|| "Hello, world!")
        .with(warp::trace::request());
    warp::serve(hello).run((Ipv6Addr::UNSPECIFIED, 8000)).await;
}
