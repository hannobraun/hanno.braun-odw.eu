mod args;
mod templates;

use templates::write_template;

use self::{
    args::{Args, Command},
    templates::Update,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Update => write_template::<Update>(())?,
    }

    Ok(())
}
