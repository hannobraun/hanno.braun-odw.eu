mod update;

pub use self::update::Update;

pub trait Template: Sized {
    fn new() -> anyhow::Result<Self>;
    fn write(&self) -> anyhow::Result<()>;
}
