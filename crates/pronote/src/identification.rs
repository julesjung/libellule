use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IndentificationData {
    pub challenge: String,
    #[serde(rename = "alea")]
    pub random: String,
}
