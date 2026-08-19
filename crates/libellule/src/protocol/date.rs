use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::protocol::Value;

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    time::macros::format_description!("[day]/[month]/[year]");

#[derive(Debug)]
pub struct RawDate(time::Date);

impl Serialize for RawDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self
            .0
            .format(DATE_FORMAT)
            .map_err(serde::ser::Error::custom)?;

        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for RawDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let date = time::Date::parse(&value, DATE_FORMAT).map_err(serde::de::Error::custom)?;

        Ok(Self(date))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Date(#[serde_as(as = "Value<7>")] RawDate);
