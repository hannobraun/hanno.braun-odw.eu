mod args;
mod update;

use self::args::{Args, Command};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Update => update::write()?,
    }

    Ok(())
}
