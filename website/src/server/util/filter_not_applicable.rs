#[derive(Debug)]
pub struct FilterNotApplicable;

impl warp::reject::Reject for FilterNotApplicable {}
