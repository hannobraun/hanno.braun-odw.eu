use warp::http::Uri;

/// Temporary redirect using a 307 response code
pub fn temporary(uri: Uri) -> impl warp::Reply {
    warp::redirect::temporary(uri)
}

/// Permanent redirect using a 308 response code
pub fn permanent(uri: Uri) -> impl warp::Reply {
    warp::reply::with_header(
        warp::redirect::permanent(uri),
        "Cache-Control",
        format!("max-age={}", 60 * 60 * 24 * 7 * 4), // 4 weeks
    )
}
