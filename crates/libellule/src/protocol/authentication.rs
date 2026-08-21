use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct AuthenticationData {
    #[serde(rename = "cle")]
    pub(crate) key: String,
}
