use std::{net::Ipv6Addr, path::PathBuf};

use clap::Clap;
use warp::{Filter as _, Reply as _};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    let port = args.port.unwrap_or(8000);
    let serve_dir = args.serve.unwrap_or("static".into());

    let hello = warp::fs::dir(serve_dir)
        .map(|file: warp::fs::File| {
            if file.path().ends_with("hello") {
                warp::reply::with_header(file, "Content-Type", "text/plain")
                    .into_response()
            } else {
                file.into_response()
            }
        })
        .with(warp::trace::request());

    warp::serve(hello)
        .tls()
        .key_path("tls/localhost.key.pem")
        .cert_path("tls/localhost.cert.pem")
        .run((Ipv6Addr::UNSPECIFIED, port))
        .await;
}

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
struct Args {
    /// Port to listen on. Will listen on port 8000, if omitted.
    #[clap(short, long)]
    port: Option<u16>,

    /// Static file directory to serve. Defaults to `static`, if omitted.
    #[clap(short, long)]
    serve: Option<PathBuf>,
}
