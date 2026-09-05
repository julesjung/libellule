use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct AuthenticationData {
    #[serde(rename = "cle")]
    pub(crate) key: Option<String>,

    #[serde(rename = "Acces", default)]
    pub(crate) failed: i32,
}
