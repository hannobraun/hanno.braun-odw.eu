use std::net::Ipv6Addr;

use clap::Clap;
use warp::Filter as _;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    Args::parse();
    let port = 8000;

    let hello = warp::any()
        .map(|| "Hello, world!")
        .with(warp::trace::request());
    warp::serve(hello).run((Ipv6Addr::UNSPECIFIED, port)).await;
}

/// Custom backend for made-be.braun-odw.eu
#[derive(Clap)]
struct Args;
