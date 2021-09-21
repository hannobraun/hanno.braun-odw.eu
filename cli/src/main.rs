mod args;
mod generators;

use self::{
    args::{Args, Command},
    generators::{generate, Note, Update},
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Note { title } => generate::<Note>(title)?,
        Command::Update => generate::<Update>(())?,
    }

    Ok(())
}
