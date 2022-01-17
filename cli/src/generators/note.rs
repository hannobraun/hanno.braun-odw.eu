use std::{
    io,
    path::{Path, PathBuf},
};

use time::{macros::format_description, OffsetDateTime};

use super::Generator;

pub struct Note {
    title: String,
    path: String,
    date: String,
}

impl Generator for Note {
    type Args = String;

    fn new(title: Self::Args) -> anyhow::Result<Self> {
        let mut path =
            title.replace(" ", "-").replace(":", "").replace("?", "");
        path.make_ascii_lowercase();

        let date_format = format_description!("[year]-[month]-[day]");
        let now = OffsetDateTime::now_utc();
        let date = now.format(&date_format)?;

        Ok(Self { title, path, date })
    }

    fn dir_path(&self) -> anyhow::Result<PathBuf> {
        let dir_path = format!("content/notes/{}", self.path);
        Ok(dir_path.into())
    }

    fn file_path(&self, dir_path: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        let file_path = dir_path.as_ref().join("index.md");
        Ok(file_path.into())
    }

    fn write(&self, mut output: impl io::Write) -> anyhow::Result<()> {
        write!(
            output,
            "\
+++
title = \"{title}\"
date  = {date}

[extra]
image = \"image.jpg\"
intro = \"\"\"
This is the introduction that is shown in note previews and the note itself.
\"\"\"
+++

This is some text.

{{{{
    preview_image(
        path=\"/notes/{path}/image.jpg\",
        alt=\"\"
    )
}}}}

{{{{ ext_link(text=\"An external link.\", url=\"https://example.com\") }}}}
\
        ",
            title = self.title,
            date = self.date,
            path = self.path,
        )?;

        Ok(())
    }
}
