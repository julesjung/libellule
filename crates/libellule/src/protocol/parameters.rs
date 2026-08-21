use serde::{Deserialize, Serialize};

use crate::protocol::ValueWrapper;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct InstanceParameters {
    #[serde(rename = "General")]
    pub(crate) general: GeneralParameters,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct GeneralParameters {
    #[serde(rename = "versionPN")]
    pub(crate) version: String,

    #[serde(rename = "NomEtablissementConnexion")]
    pub(crate) label: String,

    #[serde(rename = "PremierLundi")]
    pub(crate) first_monday: ValueWrapper<String>,

    #[serde(rename = "PremiereDate")]
    pub(crate) first_day: ValueWrapper<String>,

    #[serde(rename = "DerniereDate")]
    pub(crate) last_day: ValueWrapper<String>,

    #[serde(rename = "PlacesParJour")]
    pub(crate) places_per_day: u32,

    #[serde(rename = "PlacesParHeure")]
    pub(crate) places_per_hour: u32,

    #[serde(rename = "ListeHeures")]
    pub(crate) start_hours: ValueWrapper<Vec<Hour>>,

    #[serde(rename = "ListeHeuresFin")]
    pub(crate) _end_hours: ValueWrapper<Vec<Hour>>,

    #[serde(rename = "ListePeriodes")]
    pub(crate) periods: Vec<Period>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Hour {
    #[serde(rename = "G")]
    pub(crate) id: u32,

    #[serde(rename = "L")]
    pub(crate) label: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserParameters {
    #[serde(rename = "ressource")]
    pub(crate) resources: Resources,

    #[serde(rename = "listeInformationsEtablissements")]
    pub(crate) _institutions: ValueWrapper<Vec<Institution>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Resources {
    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "L")]
    pub(crate) label: String,

    #[serde(rename = "G")]
    pub(crate) group: u32,

    #[serde(rename = "classeDEleve")]
    pub(crate) _class: Class,

    #[serde(rename = "listeOngletsPourPeriodes")]
    pub(crate) tabs_periods: ValueWrapper<Vec<TabPeriods>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Class {
    #[serde(rename = "L")]
    pub(crate) _name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Institution {
    #[serde(rename = "L")]
    pub(crate) _name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct TabPeriods {
    #[serde(rename = "G")]
    pub(crate) id: u32,
    #[serde(rename = "listePeriodes")]
    pub(crate) periods: ValueWrapper<Vec<Period>>,
    #[serde(rename = "periodeParDefaut")]
    pub(crate) default: ValueWrapper<Period>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Period {
    #[serde(rename = "N")]
    pub(crate) id: Option<String>,

    #[serde(rename = "L")]
    pub(crate) name: String,
}
