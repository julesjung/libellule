use time::Date;
use time::macros::format_description;

use crate::error::ConversionError;
use crate::protocol;

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[day]/[month]/[year]");

pub fn inner_text(raw: protocol::Html) -> String {
    if let Ok(dom) = tl::parse(&raw.0, tl::ParserOptions::default()) {
        let parser = dom.parser();
        if let Some(first_node) = dom.children().first().and_then(|node| node.get(parser)) {
            return first_node.inner_text(parser).to_string();
        }
    }

    raw.0
}

pub fn date(raw: protocol::Date) -> Result<Date, ConversionError> {
    Date::parse(&raw.0, DATE_FORMAT).map_err(ConversionError::from)
}
