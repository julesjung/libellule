use serde::Deserialize;
use serde_with::DeserializeAs;

#[derive(Deserialize, Debug)]
pub struct Value<T> {
    #[serde(rename = "V")]
    pub value: T,
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
        Value::<T>::deserialize(deserializer).map(|value| value.value)
    }
}

#[derive(Deserialize, Debug)]
pub struct ReferencedObject {
    #[serde(rename = "N")]
    id: String,
    #[serde(rename = "L")]
    label: String,
}

#[derive(Deserialize, Debug)]
pub struct NamedObject {
    #[serde(rename = "L")]
    label: String,
}
