mod update;

fn main() -> anyhow::Result<()> {
    update::write()?;

    Ok(())
}
