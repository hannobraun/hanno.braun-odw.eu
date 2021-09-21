mod args;
mod template;
mod update;

use self::{
    args::{Args, Command},
    template::Template as _,
    update::Update,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let template = match args.command {
        Command::Update => Update,
    };

    template.write()?;

    Ok(())
}
