mod args;
mod templates;

use self::{
    args::{Args, Command},
    templates::{Template as _, Update},
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let template = match args.command {
        Command::Update => Update,
    };

    template.write()?;

    Ok(())
}
