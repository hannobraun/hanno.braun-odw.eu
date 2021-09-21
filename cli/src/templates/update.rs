use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    path::Path,
};

use time::{macros::format_description, OffsetDateTime};

use super::Template;

pub struct Update {
    title: String,
    date: String,
}

impl Template for Update {
    fn new() -> anyhow::Result<Self> {
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

    fn write(&self) -> anyhow::Result<()> {
        let dir_path = format!("content/updates/{}", self.title);
        fs::create_dir_all(&dir_path)?;

        let file_path = Path::new(&dir_path).join("index.md");
        let mut update = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file_path)?;

        write!(
            update,
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
