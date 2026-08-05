use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthenticationData {
    #[serde(rename = "cle")]
    pub key: String,
}
