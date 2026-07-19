use crate::api::Value;
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "listeOngletsPourPeriodes")]
    pub tab_periods_list: Value<Vec<TabPeriods>>,
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
    pub default_period: Value<Period>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Period {
    #[serde(rename = "N")]
    pub id: Option<String>,
    #[serde(rename = "L")]
    pub name: String,
}
