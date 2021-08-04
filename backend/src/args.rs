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
            http_port: args.http_port.unwrap_or(8080),
            https_port: args.https_port.unwrap_or(8443),
            tls_key: args.tls_key.unwrap_or("tls/localhost.key.pem".into()),
            tls_cert: args.tls_cert.unwrap_or("tls/localhost.cert.pem".into()),
            static_dir: args.static_dir.unwrap_or("static".into()),
        }
    }
}

/// Custom backend for made-by.braun-odw.eu
#[derive(Clap)]
struct CliArgs {
    /// HTTP port to listen on. Defaults to 8080, if omitted.
    #[clap(long)]
    http_port: Option<u16>,

    /// HTTPS port to listen on. Defaults to 8443, if omitted.
    #[clap(long)]
    https_port: Option<u16>,

    /// Path to TLS key file. Defaults to `tls/localhost.key.pem`, if omitted.
    #[clap(long)]
    tls_key: Option<PathBuf>,

    /// Path to TLS certificate file. Defaults to `tls/localhost.cert.pem`, if
    /// omitted.
    #[clap(long)]
    tls_cert: Option<PathBuf>,

    /// Static file directory to serve. Defaults to `static`, if omitted.
    #[clap(long)]
    static_dir: Option<PathBuf>,
}
