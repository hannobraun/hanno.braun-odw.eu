use std::{
    net::Ipv6Addr,
    path::{Path, PathBuf},
};

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

    let args = CliArgs::parse();
    let http_port = args.http_port.unwrap_or(8080);
    let https_port = args.https_port.unwrap_or(8443);
    let tls_key = args.tls_key.unwrap_or("tls/localhost.key.pem".into());
    let tls_cert = args.tls_cert.unwrap_or("tls/localhost.cert.pem".into());
    let serve_dir = args.serve.unwrap_or("static".into());

    let http_server = http_server(http_port, https_port);
    let https_server = https_server(serve_dir, tls_key, tls_cert, https_port);

    tokio::join!(https_server, http_server);
}

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
struct CliArgs {
    /// HTTP port to listen on. Defaults to 8080, if omitted.
    #[clap(long)]
    http_port: Option<u16>,

    /// HTTPS port to listen on. Defaults to 8443, if omitted.
    #[clap(long)]
    https_port: Option<u16>,

    /// Path to TLS key file. Defaults to `tls/localhost.key.pem`, if omitted.
    #[clap(long)]
    tls_key: Option<PathBuf>,

    /// Path to TLS certificate file. Defaults to `tls/localhost.cert.pem`, if
    /// omitted.
    #[clap(long)]
    tls_cert: Option<PathBuf>,

    /// Static file directory to serve. Defaults to `static`, if omitted.
    #[clap(long)]
    serve: Option<PathBuf>,
}

fn http_server(http_port: u16, https_port: u16) -> impl Future {
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

    warp::serve(redirect_to_https).run((Ipv6Addr::UNSPECIFIED, http_port))
}

fn https_server(
    serve_dir: impl Into<PathBuf> + 'static,
    tls_key: impl AsRef<Path>,
    tls_cert: impl AsRef<Path>,
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
        .key_path(tls_key)
        .cert_path(tls_cert)
        .run((Ipv6Addr::UNSPECIFIED, https_port))
}
