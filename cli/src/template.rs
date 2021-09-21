pub trait Template {
    fn write(&self) -> anyhow::Result<()>;
}
