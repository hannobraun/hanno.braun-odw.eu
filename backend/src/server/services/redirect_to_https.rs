use warp::{
    host::Authority, http::StatusCode, http::Uri, path::FullPath, Filter,
    Rejection, Reply,
};

use crate::server::util::redirect;

pub fn redirect_to_https(
    https_port: u16,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::host::optional().and(warp::path::full()).map(
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

            redirect::permanent(uri).into_response()
        },
    )
}
