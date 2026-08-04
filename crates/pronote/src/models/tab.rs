use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::Error;

#[derive(Debug, PartialEq, Hash, Deserialize_repr, Serialize_repr)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u32)]
pub enum Tab {
    Timetable = 16,
    Grades = 198,
}

impl Eq for Tab {}

impl TryFrom<u32> for Tab {
    type Error = Error;

    fn try_from(value: u32) -> Result<Tab, Error> {
        match value {
            198 => Ok(Tab::Grades),
            _ => Err(Error::UnknownTab),
        }
    }
}
