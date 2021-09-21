mod args;
mod template;
mod update;

use self::{
    args::{Args, Command},
    template::Template as _,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let template = match args.command {
        Command::Update => update::Update,
    };

    template.write()?;

    Ok(())
}
