use std::{net::Ipv6Addr, path::PathBuf};

use clap::Clap;
use warp::{
    host::Authority,
    http::{StatusCode, Uri},
    path::FullPath,
    Filter as _, Future, Reply as _,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    let https_port = args.https_port.unwrap_or(8443);
    let serve_dir = args.serve.unwrap_or("static".into());

    let http_server = http_server(https_port);
    let https_server = https_server(serve_dir, https_port);

    tokio::join!(https_server, http_server);
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

fn http_server(https_port: u16) -> impl Future {
    let redirect_to_https = warp::host::optional().and(warp::path::full()).map(
        move |authority: Option<Authority>, path: FullPath| {
            let authority = match authority {
                Some(authority) => authority,
                None => {
                    return warp::reply::with_status(
                        "Could not extract authority from request.",
                        StatusCode::BAD_REQUEST,
                    )
                    .into_response()
                }
            };

            let authority: Authority =
                format!("{}:{}", authority.host(), https_port)
                    .parse()
                    // Should never happen, unless the `format!` call above is
                    // buggy.
                    .expect("Failed to parse authority.");

            let uri = Uri::builder()
                .scheme("https")
                .authority(authority)
                .path_and_query(path.as_str())
                .build()
                // Should never happen, unless invalid arguments are passed to
                // the builder methods above, which would be a bug.
                .expect("Failed to build URI");

            warp::redirect(uri).into_response()
        },
    );

    warp::serve(redirect_to_https).run((Ipv6Addr::UNSPECIFIED, 8080))
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
