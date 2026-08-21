use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct IndentificationData {
    pub(crate) challenge: String,

    #[serde(rename = "alea")]
    pub(crate) random: String,
}
