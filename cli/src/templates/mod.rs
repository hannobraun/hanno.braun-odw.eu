mod update;

use std::{
    io,
    path::{Path, PathBuf},
};

pub use self::update::Update;

pub trait Template: Sized {
    fn new() -> anyhow::Result<Self>;
    fn dir_path(&self) -> anyhow::Result<PathBuf>;
    fn file_path(&self, dir_path: impl AsRef<Path>) -> anyhow::Result<PathBuf>;
    fn write(&self, output: impl io::Write) -> anyhow::Result<()>;
}
