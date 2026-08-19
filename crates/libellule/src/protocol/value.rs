use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, Visitor},
};
use serde_with::{DeserializeAs, SerializeAs, serde_as};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueWrapper<T> {
    #[serde(rename = "_T")]
    pub kind: u32,

    #[serde(rename = "V")]
    pub value: T,
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
pub struct Weeks(#[serde_as(as = "Value<8>")] String);

impl Weeks {
    pub fn from_single(week: u32) -> Weeks {
        Weeks(format!("[{week}]"))
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Html(#[serde_as(as = "Value<21>")] String);

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Array<T>(#[serde_as(as = "Value<24>")] Vec<T>);

impl<T> From<Array<T>> for Vec<T> {
    fn from(value: Array<T>) -> Self {
        value.0
    }
}
impl<T> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        Array(value)
    }
}

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a hex color such as #FF8800")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let hex = value.strip_prefix('#').unwrap_or(value);

                if hex.len() != 6 {
                    return Err(E::custom("invalid hex color code"));
                }

                let red = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| E::custom("invalid red component"))?;
                let green = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| E::custom("invalid green component"))?;
                let blue = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| E::custom("invalid blue component"))?;

                Ok(Color { red, green, blue })
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}
