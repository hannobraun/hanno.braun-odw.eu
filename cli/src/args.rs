#[derive(clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

impl Args {
    /// Convenience method to ease access to `Clap::parse` implementation
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Parser)]
pub enum Command {
    Update,
    Note { title: String },
}
