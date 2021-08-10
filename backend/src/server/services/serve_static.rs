use std::path::PathBuf;

use serde::Deserialize;
use warp::{
    http::{Response, StatusCode},
    hyper::Body,
    reply, Filter, Rejection, Reply,
};

use crate::server::util::{handle_not_found, resize_image};

pub fn serve_static(
    static_dir: impl Into<PathBuf> + 'static,
    zola_dir: impl Into<PathBuf> + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::fs::dir(static_dir)
        .or(warp::fs::dir(zola_dir))
        .unify()
        .and(warp::query::<ImageArgs>())
        .map(|file: warp::fs::File, args: ImageArgs| {
            if let Some(width) = args.width {
                if file.path().as_os_str().to_string_lossy().ends_with(".jpg") {
                    match resize_image(file.path(), width) {
                        Ok(image) => {
                            return Response::new(Body::from(image));
                        }
                        Err(err) => {
                            return reply::with_status(
                                err.to_string(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            )
                            .into_response();
                        }
                    }
                }
            }

            file.into_response()
        })
        .recover(handle_not_found)
}

#[derive(Debug, Deserialize)]
struct ImageArgs {
    width: Option<u32>,
}
