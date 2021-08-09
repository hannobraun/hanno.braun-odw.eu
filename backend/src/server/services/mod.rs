mod redirect_home;
mod redirect_legacy_domain;
mod serve_static;

pub use self::{
    redirect_home::redirect_home,
    redirect_legacy_domain::redirect_legacy_domain, serve_static::serve_static,
};
