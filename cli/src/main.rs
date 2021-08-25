use time::{macros::format_description, OffsetDateTime};

fn main() {
    // Date and time formats follow ISO 8601. See Wikipedia:
    // See https://en.wikipedia.org/wiki/ISO_8601.
    let title_format =
        format_description!("[year][month][day]T[hour][minute][second]Z");
    let date_format = format_description!("[year]-[month]-[day]");

    let now = OffsetDateTime::now_utc();

    // TASK: Improve error handling.
    let title = now.format(&title_format).unwrap();
    let date = now.format(&&date_format).unwrap();

    println!("Title: {}", title);
    println!("Date: {}", date);
}
