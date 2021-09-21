mod args;
mod templates;

use std::fs::{self, OpenOptions};

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

    let file_path = template.file_path(dir_path)?;
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(file_path)?;

    template.write(file)?;

    Ok(())
}
