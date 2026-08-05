use time::PlainDateTime;
use time::macros::format_description;

const FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[day]/[month]/[year] [hour]:[minute]:[second]");

pub fn parse_datetime(string: &str) -> Result<PlainDateTime, time::error::Parse> {
    PlainDateTime::parse(string, FORMAT)
}

pub fn format_datetime(datetime: PlainDateTime) -> Result<String, time::error::Format> {
    datetime.format(FORMAT)
}
