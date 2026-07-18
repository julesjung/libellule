use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Parameters {
    #[serde(rename = "General")]
    pub general: GeneralParameters,
}

#[derive(Deserialize, Debug)]
pub struct GeneralParameters {
    #[serde(rename = "NomEtablissement")]
    pub name: String,
    #[serde(rename = "versionPN")]
    pub version: String,
}
