use warp::{http::Uri, Filter, Rejection, Reply};

use crate::server::util::redirect;

pub fn redirect_path(
    old_path: &'static str,
    new_path: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(old_path)
        .map(move || redirect::permanent(Uri::from_static(new_path)))
}
