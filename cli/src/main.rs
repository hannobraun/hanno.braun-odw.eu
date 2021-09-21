mod args;
mod templates;

use std::fs;

use self::{
    args::{Args, Command},
    templates::{Template as _, Update},
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let template = match args.command {
        Command::Update => Update::new()?,
    };

    let dir_path = template.dir_path()?;
    fs::create_dir_all(&dir_path)?;

    template.write(dir_path)?;

    Ok(())
}
