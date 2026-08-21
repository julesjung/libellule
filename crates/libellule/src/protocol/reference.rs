use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ObjectReference {
    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "L")]
    pub(crate) name: String,
}
