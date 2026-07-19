use crate::api::function::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserParameters {
    #[serde(rename = "ressource")]
    pub resources: Resources,
    #[serde(rename = "listeInformationsEtablissements")]
    pub institution: Value<Vec<Institution>>,
}

#[derive(Deserialize, Debug)]
pub struct Resources {
    #[serde(rename = "L")]
    pub name: String,
    #[serde(rename = "classeDEleve")]
    pub class: Class,
}

#[derive(Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "L")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Institution {
    #[serde(rename = "L")]
    pub name: String,
}
