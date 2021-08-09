use warp::{path::FullPath, Filter, Rejection, Reply};

use crate::server::util::redirect;

pub fn redirect_legacy_path(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("project")
        .and(warp::path::full())
        .map(|path: FullPath| {
            let suffix = path
                .as_str()
                .split_once("/project")
                // Can't fail, unless the filters above no longer match on
                // `/project`.
                .expect("Path was missing prefix, despite matching on it")
                .1;

            let new_path = format!("/projects{}", suffix);
            let new_path = new_path
                .parse()
                // Can't happen, unless the `format!` above is buggy.
                .expect("Failed to parse URI");

            redirect::permanent(new_path)
        })
}
