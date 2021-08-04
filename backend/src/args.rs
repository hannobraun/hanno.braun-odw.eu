use std::path::PathBuf;

use clap::Clap;

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
pub struct Args {
    /// HTTP port to listen on
    #[clap(long, default_value = "8080")]
    pub http_port: u16,

    /// HTTPS port to listen on
    #[clap(long, default_value = "8443")]
    pub https_port: u16,

    /// Path to TLS key file
    #[clap(long, default_value = "tls/localhost.key.pem")]
    pub tls_key: PathBuf,

    /// Path to TLS certificate file
    #[clap(long, default_value = "tls/localhost.cert.pem")]
    pub tls_cert: PathBuf,

    /// Path to static files
    #[clap(long, default_value = "static")]
    pub static_dir: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}
