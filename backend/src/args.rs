use std::path::PathBuf;

use clap::Clap;

pub struct Args {
    pub http_port: u16,
    pub https_port: u16,
    pub tls_key: PathBuf,
    pub tls_cert: PathBuf,
    pub static_dir: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        let args = CliArgs::parse();

        Self {
            http_port: args.http_port,
            https_port: args.https_port,
            tls_key: args.tls_key,
            tls_cert: args.tls_cert,
            static_dir: args.static_dir,
        }
    }
}

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
struct CliArgs {
    /// HTTP port to listen on. Defaults to 8080, if omitted.
    #[clap(long, default_value = "8080")]
    http_port: u16,

    /// HTTPS port to listen on. Defaults to 8443, if omitted.
    #[clap(long, default_value = "8443")]
    https_port: u16,

    /// Path to TLS key file. Defaults to `tls/localhost.key.pem`, if omitted.
    #[clap(long, default_value = "tls/localhost.key.pem")]
    tls_key: PathBuf,

    /// Path to TLS certificate file. Defaults to `tls/localhost.cert.pem`, if
    /// omitted.
    #[clap(long, default_value = "tls/localhost.cert.pem")]
    tls_cert: PathBuf,

    /// Path to static files. Defaults to `static`, if omitted.
    #[clap(long, default_value = "static")]
    static_dir: PathBuf,
}
