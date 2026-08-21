use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::ConversionError;

#[derive(Debug, PartialEq, Hash, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum Tab {
    Menu = 10,
    Timetable = 16,
    Homework = 88,
    Grades = 198,
}

impl Eq for Tab {}

impl TryFrom<u32> for Tab {
    type Error = ConversionError;

    fn try_from(value: u32) -> Result<Tab, ConversionError> {
        match value {
            198 => Ok(Tab::Grades),
            _ => Err(ConversionError::Parse),
        }
    }
}
