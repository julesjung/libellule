use time::macros::format_description;
use time::{Date, PlainDateTime, Time};

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[day]/[month]/[year]");

const DATETIME_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[day]/[month]/[year] [hour]:[minute]:[second]");

const TIME_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[hour]h[minute]");

pub fn parse_date(input: &str) -> Result<Date, time::error::Parse> {
    Date::parse(input, DATE_FORMAT)
}

pub fn format_date(date: Date) -> String {
    date.format(DATE_FORMAT).unwrap()
}

pub fn parse_datetime(string: &str) -> Result<PlainDateTime, time::error::Parse> {
    PlainDateTime::parse(string, DATETIME_FORMAT)
}

pub fn format_datetime(datetime: PlainDateTime) -> String {
    datetime.format(DATETIME_FORMAT).unwrap()
}

pub fn parse_time(input: &str) -> Result<Time, time::error::Parse> {
    Time::parse(input, TIME_FORMAT)
}
