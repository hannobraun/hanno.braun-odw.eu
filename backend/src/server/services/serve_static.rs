use std::path::PathBuf;

use warp::{Filter, Rejection, Reply};

use crate::server::util::handle_not_found;

pub fn serve_static(
    static_dir: impl Into<PathBuf> + 'static,
    zola_dir: impl Into<PathBuf> + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::fs::dir(static_dir)
        .or(warp::fs::dir(zola_dir))
        .recover(handle_not_found)
        .with(warp::trace::request())
}
