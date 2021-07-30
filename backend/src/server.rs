use std::{
    net::Ipv6Addr,
    path::{Path, PathBuf},
};

use warp::{
    host::Authority,
    http::{StatusCode, Uri},
    path::FullPath,
    Filter as _, Future, Reply as _,
};

use crate::args::Args;

pub async fn server(args: Args) {
    let http_server = http_server(args.http_port, args.https_port);
    let https_server = https_server(
        args.serve_dir,
        args.tls_key,
        args.tls_cert,
        args.https_port,
    );

    tokio::join!(https_server, http_server);
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
    let redirect = warp::path::end()
        .map(|| warp::redirect::temporary(Uri::from_static("/updates")));

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

    warp::serve(redirect.or(hello))
        .tls()
        .key_path(tls_key)
        .cert_path(tls_cert)
        .run((Ipv6Addr::UNSPECIFIED, https_port))
}
