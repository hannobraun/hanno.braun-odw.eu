use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

impl Args {
    /// Convenience method to ease access to `Clap::parse` implementation
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}

#[derive(Clap)]
pub enum Command {
    Update,
    Note { title: String },
}
