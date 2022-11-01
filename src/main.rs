use std::process::Command;

fn main() -> anyhow::Result<()> {
    Command::new("zola").arg("build").status()?;
    Ok(())
}
