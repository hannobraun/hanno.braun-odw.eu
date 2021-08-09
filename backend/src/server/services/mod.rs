mod redirect_home;
mod redirect_legacy_domain;

pub use self::{
    redirect_home::redirect_home,
    redirect_legacy_domain::redirect_legacy_domain,
};
