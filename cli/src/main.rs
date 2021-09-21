mod update;

use clap::Clap;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Update => update::write()?,
    }

    Ok(())
}

#[derive(Clap)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    Update,
}
