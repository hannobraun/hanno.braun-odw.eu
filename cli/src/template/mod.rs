mod update;

pub use self::update::Update;

pub trait Template {
    fn write(&self) -> anyhow::Result<()>;
}
