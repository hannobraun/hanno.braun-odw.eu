use time::{macros::format_description, OffsetDateTime};

fn main() -> anyhow::Result<()> {
    // Date and time formats follow ISO 8601. See Wikipedia:
    // See https://en.wikipedia.org/wiki/ISO_8601.
    let title_format =
        format_description!("[year][month][day]T[hour][minute][second]Z");
    let date_format = format_description!("[year]-[month]-[day]");

    let now = OffsetDateTime::now_utc();

    let title = now.format(&title_format)?;
    let date = now.format(&&date_format)?;

    println!("Title: {}", title);
    println!("Date: {}", date);

    Ok(())
}
