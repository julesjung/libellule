use serde::{Deserialize, Serialize};

use crate::protocol::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct InstanceParameters {
    #[serde(rename = "General")]
    pub general: GeneralParameters,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeneralParameters {
    #[serde(rename = "versionPN")]
    pub version: String,

    #[serde(rename = "NomEtablissementConnexion")]
    pub label: String,

    #[serde(rename = "PremierLundi")]
    pub first_monday: Value<String>,

    #[serde(rename = "PremiereDate")]
    pub first_day: Value<String>,

    #[serde(rename = "DerniereDate")]
    pub last_day: Value<String>,

    #[serde(rename = "PlacesParJour")]
    pub places_per_day: u32,

    #[serde(rename = "PlacesParHeure")]
    pub places_per_hour: u32,

    #[serde(rename = "ListeHeures")]
    pub start_hours: Value<Vec<Hour>>,

    #[serde(rename = "ListeHeuresFin")]
    pub end_hours: Value<Vec<Hour>>,

    #[serde(rename = "ListePeriodes")]
    pub periods: Vec<Period>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hour {
    #[serde(rename = "G")]
    pub id: u32,

    #[serde(rename = "L")]
    pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct UserParameters {
    #[serde(rename = "ressource")]
    pub resources: Resources,
    #[serde(rename = "listeInformationsEtablissements")]
    pub institutions: Value<Vec<Institution>>,
}

#[derive(Deserialize, Debug)]
pub struct Resources {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "L")]
    pub label: String,
    #[serde(rename = "G")]
    pub group: u32,
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
