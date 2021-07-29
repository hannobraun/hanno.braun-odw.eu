use std::{net::Ipv6Addr, path::PathBuf};

use clap::Clap;
use warp::{Filter as _, Future, Reply as _};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    let https_port = args.https_port.unwrap_or(8443);
    let serve_dir = args.serve.unwrap_or("static".into());

    https_server(serve_dir, https_port).await;
}

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
struct Args {
    /// Port to listen on. Will listen on port 8443, if omitted.
    #[clap(short, long)]
    https_port: Option<u16>,

    /// Static file directory to serve. Defaults to `static`, if omitted.
    #[clap(short, long)]
    serve: Option<PathBuf>,
}

fn https_server(
    serve_dir: impl Into<PathBuf> + 'static,
    https_port: u16,
) -> impl Future {
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
        .run((Ipv6Addr::UNSPECIFIED, https_port))
}
