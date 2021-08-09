mod services;
mod util;

use std::{
    net::Ipv6Addr,
    path::{Path, PathBuf},
};

use warp::Filter as _;

use crate::args::Args;

use self::{
    services::{
        redirect_home, redirect_legacy_domain, redirect_path,
        redirect_path_prefix, redirect_to_https, serve_static,
    },
    util::redirect,
};

pub async fn server(args: Args) {
    tokio::join!(
        http_server(args.http_port, args.https_port),
        https_server(
            args.static_dir,
            args.zola_dir,
            args.tls_key,
            args.tls_cert,
            args.https_port,
        )
    );
}

fn http_server(http_port: u16, https_port: u16) -> impl warp::Future {
    warp::serve(redirect_to_https(https_port))
        .run((Ipv6Addr::UNSPECIFIED, http_port))
}

fn https_server(
    static_dir: impl Into<PathBuf> + 'static,
    zola_dir: impl Into<PathBuf> + 'static,
    tls_key: impl AsRef<Path>,
    tls_cert: impl AsRef<Path>,
    https_port: u16,
) -> impl warp::Future {
    warp::serve(
        redirect_legacy_domain()
            .or(redirect_home())
            .or(redirect_path("rss.xml", "/atom.xml"))
            .or(redirect_path_prefix("project", "projects"))
            .or(serve_static(static_dir, zola_dir)),
    )
    .tls()
    .key_path(tls_key)
    .cert_path(tls_cert)
    .run((Ipv6Addr::UNSPECIFIED, https_port))
}
