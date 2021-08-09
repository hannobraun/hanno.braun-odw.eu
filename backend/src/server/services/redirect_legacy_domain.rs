use warp::{
    host::Authority,
    http::{uri::Scheme, Uri},
    path::FullPath,
    Filter, Rejection, Reply,
};

use crate::server::{redirect, util::FilterNotApplicable};

pub fn redirect_legacy_domain(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::host::optional().and(warp::path::full()).and_then(
        |authority: Option<Authority>, path: FullPath| async move {
            if let Some("madeby.hannobraun.de") =
                authority.as_ref().map(|a| a.host())
            {
                return Ok(redirect::permanent(
                    Uri::builder()
                        .scheme(Scheme::HTTPS)
                        .authority(Authority::from_static(
                            "made-by.braun-odw.eu",
                        ))
                        .path_and_query(path.as_str())
                        .build()
                        // Shouldn't happen, unless the parameters above are
                        // invalid.
                        .expect("invalid URI"),
                ));
            }

            // This is not the legacy domain. This filter isn't responsible for
            // this request.
            Err(warp::reject::custom(FilterNotApplicable))
        },
    )
}
