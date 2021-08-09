use warp::http::Uri;

/// Temporary redirect using a 307 response code
pub fn temporary(uri: Uri) -> impl warp::Reply {
    warp::redirect::temporary(uri)
}

/// Permanent redirect using a 308 response code
pub fn permanent(uri: Uri) -> impl warp::Reply {
    warp::redirect::permanent(uri)
}
