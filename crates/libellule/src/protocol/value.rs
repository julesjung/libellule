use serde::{Deserialize, Serialize};
use serde_with::{DeserializeAs, SerializeAs, serde_as};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueWrapper<T> {
    #[serde(rename = "_T")]
    pub kind: u32,

    #[serde(rename = "V")]
    pub value: T,
}

pub struct Value<const KIND: u32>;

impl<T, const KIND: u32> SerializeAs<T> for Value<KIND>
where
    T: Serialize,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = ValueWrapper {
            kind: KIND,
            value: source,
        };

        serializer.serialize_some(&value)
    }
}

impl<'de, T, const KIND: u32> DeserializeAs<'de, T> for Value<KIND>
where
    T: Deserialize<'de>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = ValueWrapper::<T>::deserialize(deserializer)?;

        if value.kind == KIND {
            Ok(value.value)
        } else {
            let error = format!("unexpected value kind `{}`", value.kind);
            Err(serde::de::Error::custom(error))
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Weeks(#[serde_as(as = "Value<8>")] u32);

impl Weeks {
    pub fn from_single(week: u32) -> Weeks {
        Weeks(week)
    }
}

#[deprecated]
pub struct FromUncheckedValue;

impl<'de, T> DeserializeAs<'de, T> for FromUncheckedValue
where
    T: Deserialize<'de>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ValueWrapper::<T>::deserialize(deserializer).map(|value| value.value)
    }
}
