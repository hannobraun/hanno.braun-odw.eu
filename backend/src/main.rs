use std::net::Ipv6Addr;

use warp::Filter as _;

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| "Hello, world!");
    warp::serve(hello).run((Ipv6Addr::UNSPECIFIED, 8000)).await;
}
