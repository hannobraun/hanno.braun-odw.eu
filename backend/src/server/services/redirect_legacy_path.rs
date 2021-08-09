use warp::{path::FullPath, Filter, Rejection, Reply};

use crate::server::util::redirect;

pub fn redirect_legacy_path(
    old_prefix: &'static str,
    new_prefix: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(old_prefix)
        .and(warp::path::full())
        .map(move |path: FullPath| {
            let suffix = path
                .as_str()
                .split_once(old_prefix)
                // Can't fail, unless the filters above no longer match on the
                // old prefix.
                .expect("Path was missing prefix, despite matching on it")
                .1;

            let new_path = format!("/{}{}", new_prefix, suffix);
            let new_path = new_path
                .parse()
                // Can't happen, unless the `format!` above is buggy.
                .expect("Failed to parse URI");

            redirect::permanent(new_path)
        })
}
