use serde::Deserialize;
use serde_with::DeserializeAs;

#[derive(Deserialize, Debug)]
pub struct Value<T> {
    #[serde(rename = "V")]
    value: T,
}

pub struct FromValue;

impl<'de, T> DeserializeAs<'de, T> for FromValue
where
    T: Deserialize<'de>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Value::<T>::deserialize(deserializer)?.value)
    }
}
