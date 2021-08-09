use warp::http::Uri;

/// Temporary redirect using a 307 response code
pub fn temporary(uri: Uri) -> impl warp::Reply {
    warp::redirect::temporary(uri)
}

/// Permanent redirect using a 308 response code
pub fn permanent(uri: Uri) -> impl warp::Reply {
    with_cache_control_header(
        warp::redirect::permanent(uri),
        time::Duration::weeks(4),
    )
}

/// Adds a `Cache-Control` header to the given response
///
/// # Panics
///
/// Panics, if `max_age` is negative.
fn with_cache_control_header(
    reply: impl warp::Reply,
    max_age: time::Duration,
) -> impl warp::Reply {
    assert!(max_age.is_positive());

    warp::reply::with_header(
        reply,
        "Cache-Control",
        format!("max-age={}", max_age.whole_seconds()),
    )
}
