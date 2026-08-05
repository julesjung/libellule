use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Subject {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "L")]
    pub name: String,
}
