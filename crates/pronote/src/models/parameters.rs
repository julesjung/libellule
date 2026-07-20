use crate::api::FromValue;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct UserParameters {
    #[serde(rename = "ressource")]
    pub resources: Resources,
    #[serde(rename = "listeInformationsEtablissements")]
    #[serde_as(as = "FromValue")]
    pub institution: Vec<Institution>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Resources {
    #[serde(rename = "L")]
    pub name: String,
    #[serde(rename = "classeDEleve")]
    pub class: Class,
    #[serde(rename = "listeOngletsPourPeriodes")]
    #[serde_as(as = "FromValue")]
    pub tab_periods_list: Vec<TabPeriods>,
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

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TabPeriods {
    #[serde(rename = "G")]
    pub id: u32,
    #[serde(rename = "listePeriodes")]
    #[serde_as(as = "FromValue")]
    pub periods: Vec<Period>,
    #[serde(rename = "periodeParDefaut")]
    #[serde_as(as = "FromValue")]
    pub default_period: Period,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Period {
    #[serde(rename = "N")]
    pub id: Option<String>,
    #[serde(rename = "L")]
    pub name: String,
}
