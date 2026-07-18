use serde::Serialize;

#[derive(Serialize)]
pub enum Function {
    #[serde(rename = "FonctionParametres")]
    InstanceParameters,
}
