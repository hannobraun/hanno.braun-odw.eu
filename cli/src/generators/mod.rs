mod note;
mod update;

use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

pub use self::{note::Note, update::Update};

pub trait Generator: Sized {
    type Args;

    fn new(args: Self::Args) -> anyhow::Result<Self>;
    fn dir_path(&self) -> anyhow::Result<PathBuf>;
    fn file_path(&self, dir_path: impl AsRef<Path>) -> anyhow::Result<PathBuf>;
    fn write(&self, output: impl io::Write) -> anyhow::Result<()>;
}

pub fn generate<G: Generator>(args: G::Args) -> anyhow::Result<()> {
    let template = G::new(args)?;

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
