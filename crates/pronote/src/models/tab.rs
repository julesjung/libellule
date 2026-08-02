use crate::error::Error;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum Tab {
    Grades,
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
