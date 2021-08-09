mod redirect_home;
mod redirect_legacy_domain;
mod redirect_to_https;
mod serve_static;

pub use self::{
    redirect_home::redirect_home,
    redirect_legacy_domain::redirect_legacy_domain,
    redirect_to_https::redirect_to_https, serve_static::serve_static,
};
