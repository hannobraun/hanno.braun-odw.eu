use std::net::Ipv6Addr;

use clap::Clap;
use warp::Filter as _;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    let port = args.port.unwrap_or(8000);

    let hello = warp::any()
        .map(|| "Hello, world!")
        .with(warp::trace::request());
    warp::serve(hello).run((Ipv6Addr::UNSPECIFIED, port)).await;
}

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
struct Args {
    /// Port to listen on. Will listen on port 8000, if omitted.
    #[clap(short, long)]
    port: Option<u16>,
}
