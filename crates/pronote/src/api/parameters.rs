use serde::{Deserialize, Serialize};

use crate::api::Value;

#[derive(Deserialize, Debug)]
pub struct UserParameters {
    #[serde(rename = "ressource")]
    pub resources: Resources,
    #[serde(rename = "listeInformationsEtablissements")]
    pub institutions: Value<Vec<Institution>>,
}

#[derive(Deserialize, Debug)]
pub struct Resources {
    #[serde(rename = "L")]
    pub label: String,
    #[serde(rename = "classeDEleve")]
    pub class: Class,
    #[serde(rename = "listeOngletsPourPeriodes")]
    pub tabs_periods: Value<Vec<TabPeriods>>,
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

#[derive(Deserialize, Debug)]
pub struct TabPeriods {
    #[serde(rename = "G")]
    pub id: u32,
    #[serde(rename = "listePeriodes")]
    pub periods: Value<Vec<Period>>,
    #[serde(rename = "periodeParDefaut")]
    pub default: Value<Period>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Period {
    #[serde(rename = "N")]
    pub id: Option<String>,
    #[serde(rename = "L")]
    pub name: String,
}
