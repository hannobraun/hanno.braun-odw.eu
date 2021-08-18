use warp::{path::FullPath, Filter, Rejection, Reply};

use crate::server::util::{redirect, FilterNotApplicable};

pub fn remove_trailing_slash(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::full().and_then(|path: FullPath| async move {
        if let Some((path, "")) = path.as_str().rsplit_once("/") {
            let path = path
                .parse()
                // This should never happen. What we get out of a `FullPath`,
                // without the trailing slash, should always be a valid URI.
                .expect("Failed to parse path to URI");

            return Ok(redirect::permanent(path));
        }

        // This request does not end with a trailing slash.
        Err(warp::reject::custom(FilterNotApplicable))
    })
}
