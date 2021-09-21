use std::{
    io,
    path::{Path, PathBuf},
};

use time::{macros::format_description, OffsetDateTime};

use super::Generator;

pub struct Update {
    title: String,
    date: String,
}

impl Generator for Update {
    type Args = ();

    fn new(_: Self::Args) -> anyhow::Result<Self> {
        // Date and time formats follow ISO 8601. See Wikipedia:
        // See https://en.wikipedia.org/wiki/ISO_8601.
        let title_format =
            format_description!("[year][month][day]T[hour][minute][second]Z");
        let date_format = format_description!("[year]-[month]-[day]");

        let now = OffsetDateTime::now_utc();

        let title = now.format(&title_format)?;
        let date = now.format(&date_format)?;

        Ok(Update { title, date })
    }

    fn dir_path(&self) -> anyhow::Result<PathBuf> {
        let dir_path = format!("content/updates/{}", self.title);
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
date  = \"{date}\"
+++

{{{{
    preview_image(
        path=\"updates/{title}/image.jpg\",
        alt=\"\"
    )
}}}}

{{{{ ext_link(url=\"https://example.com\", text=\"An external link.\") }}}}

This is an update.
\
        ",
            title = self.title,
            date = self.date,
        )?;

        Ok(())
    }
}
