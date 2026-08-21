use serde::Deserialize;

use crate::protocol::{Array, Color, Date, Html, Object, ObjectReference};

#[derive(Debug, Deserialize)]
pub(crate) struct Homework {
    #[serde(rename = "ListeTravauxAFaire")]
    pub(crate) tasks: Array<HomeworkItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HomeworkItem {
    #[serde(rename = "CouleurFond")]
    pub(crate) background: Color,

    #[serde(rename = "DonneLe")]
    pub(crate) given_on: Date,

    #[serde(rename = "Matiere")]
    pub(crate) subject: Object<ObjectReference>,

    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "PourLe")]
    pub(crate) due: Date,

    #[serde(rename = "TAFFait")]
    pub(crate) done: bool,

    #[serde(rename = "descriptif")]
    pub(crate) description: Html,
}
