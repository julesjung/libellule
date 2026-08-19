use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReference {
    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "L")]
    pub name: String,
}
