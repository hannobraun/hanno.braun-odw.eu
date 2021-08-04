use std::{
    net::Ipv6Addr,
    path::{Path, PathBuf},
};

use warp::{
    host::Authority,
    http::{StatusCode, Uri},
    path::FullPath,
    reply, Filter as _, Future, Rejection, Reply,
};

use crate::args::Args;

pub async fn server(args: Args) {
    let http_server = http_server(args.http_port, args.https_port);
    let https_server = https_server(
        args.static_dir,
        args.zola_dir,
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
    static_dir: impl Into<PathBuf> + 'static,
    zola_dir: impl Into<PathBuf> + 'static,
    tls_key: impl AsRef<Path>,
    tls_cert: impl AsRef<Path>,
    https_port: u16,
) -> impl Future {
    let redirect_to_updates = warp::path::end()
        .map(|| warp::redirect::temporary(Uri::from_static("/updates")));

    let serve_static = warp::fs::dir(static_dir)
        .or(warp::fs::dir(zola_dir))
        .recover(handle_not_found)
        .with(warp::trace::request());

    warp::serve(redirect_to_updates.or(serve_static))
        .tls()
        .key_path(tls_key)
        .cert_path(tls_cert)
        .run((Ipv6Addr::UNSPECIFIED, https_port))
}

async fn handle_not_found(
    rejection: Rejection,
) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        return Ok(reply::with_status("not found", StatusCode::NOT_FOUND));
    }

    Err(rejection)
}
