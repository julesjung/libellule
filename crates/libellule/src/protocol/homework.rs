use serde::Deserialize;

use crate::protocol::{Array, Color, Date, Html, Object, ObjectReference};

#[derive(Debug, Deserialize)]
pub struct Homework {
    #[serde(rename = "ListeTravauxAFaire")]
    pub tasks: Array<HomeworkItem>,
}

#[derive(Debug, Deserialize)]
pub struct HomeworkItem {
    #[serde(rename = "CouleurFond")]
    pub background: Color,

    #[serde(rename = "DonneLe")]
    pub given_on: Date,

    #[serde(rename = "Matiere")]
    pub subject: Object<ObjectReference>,

    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "PourLe")]
    pub due: Date,

    #[serde(rename = "TAFFait")]
    pub done: bool,

    #[serde(rename = "descriptif")]
    pub description: Html,
}
