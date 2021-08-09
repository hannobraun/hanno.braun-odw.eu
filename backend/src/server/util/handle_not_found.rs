pub async fn handle_not_found(
    rejection: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if rejection.is_not_found() {
        return Ok(warp::reply::with_status(
            "not found",
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    Err(rejection)
}
