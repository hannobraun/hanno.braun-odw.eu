use warp::{http::Uri, Filter, Rejection, Reply};

use crate::server::util::redirect;

pub fn redirect_home(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().map(|| redirect::temporary(Uri::from_static("/updates")))
}
