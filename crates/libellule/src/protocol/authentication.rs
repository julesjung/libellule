use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct AuthenticationData {
    #[serde(rename = "cle")]
    pub(crate) key: Option<String>,

    #[serde(rename = "Access")]
    pub(crate) failed: i32,
}
