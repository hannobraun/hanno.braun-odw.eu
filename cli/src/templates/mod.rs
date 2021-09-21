mod update;

use std::path::{Path, PathBuf};

pub use self::update::Update;

pub trait Template: Sized {
    fn new() -> anyhow::Result<Self>;
    fn dir_path(&self) -> anyhow::Result<PathBuf>;
    fn write(&self, dir_path: impl AsRef<Path>) -> anyhow::Result<()>;
}
