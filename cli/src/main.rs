mod args;
mod templates;

use self::{
    args::{Args, Command},
    templates::{write_template, Note, Update},
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Note { title } => write_template::<Note>(title)?,
        Command::Update => write_template::<Update>(())?,
    }

    Ok(())
}
