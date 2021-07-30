mod args;
mod server;

use crate::{args::Args, server::server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    server(args).await;
}
