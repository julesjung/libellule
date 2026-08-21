use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use time::macros::format_description;

use crate::error::ProtocolError;

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[day]/[month]/[year]");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ValueWrapper<T> {
    #[serde(rename = "_T")]
    pub kind: i32,

    #[serde(rename = "V")]
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub(crate) struct Color(pub(crate) String);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "ValueWrapper<String>", try_from = "ValueWrapper<String>")]
pub(crate) struct Date(pub(crate) String);

impl From<Date> for ValueWrapper<String> {
    fn from(value: Date) -> Self {
        ValueWrapper {
            kind: 7,
            value: value.0,
        }
    }
}

impl TryFrom<ValueWrapper<String>> for Date {
    type Error = ProtocolError;

    fn try_from(value: ValueWrapper<String>) -> Result<Self, Self::Error> {
        if value.kind == 7 {
            return Ok(Date(value.value));
        }

        Err(ProtocolError::UnexpectedValueKind {
            kind: value.kind,
            expected: 7,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "ValueWrapper<String>", try_from = "ValueWrapper<String>")]
pub(crate) struct Html(pub(crate) String);

impl From<Html> for ValueWrapper<String> {
    fn from(value: Html) -> Self {
        ValueWrapper {
            kind: 21,
            value: value.0,
        }
    }
}

impl TryFrom<ValueWrapper<String>> for Html {
    type Error = ProtocolError;

    fn try_from(value: ValueWrapper<String>) -> Result<Self, Self::Error> {
        if value.kind == 21 {
            return Ok(Html(value.value));
        }

        Err(ProtocolError::UnexpectedValueKind {
            kind: value.kind,
            expected: 21,
        })
    }
}

pub type Array<T> = Object<Vec<T>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(
    into = "ValueWrapper<T>",
    try_from = "ValueWrapper<T>",
    bound(
        serialize = "T: Clone + Serialize",
        deserialize = "T: DeserializeOwned"
    )
)]
pub(crate) struct Object<T>(pub(crate) T);

impl<T> From<Object<T>> for ValueWrapper<T> {
    fn from(value: Object<T>) -> Self {
        ValueWrapper {
            kind: 21,
            value: value.0,
        }
    }
}

impl<T> TryFrom<ValueWrapper<T>> for Object<T> {
    type Error = ProtocolError;

    fn try_from(value: ValueWrapper<T>) -> Result<Self, Self::Error> {
        if value.kind == 24 {
            return Ok(Object(value.value));
        }

        Err(ProtocolError::UnexpectedValueKind {
            kind: value.kind,
            expected: 24,
        })
    }
}
