use serde::Deserialize;

use crate::protocol::{Array, Color, Date, Html, ObjectReference, ValueWrapper};

#[derive(Debug, Deserialize)]
pub struct Homework {
    #[serde(rename = "ListeTravauxAFaire")]
    pub tasks: Array<Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    #[serde(rename = "CouleurFond")]
    pub background: Color,

    #[serde(rename = "DonneLe")]
    pub given_on: Date,

    #[serde(rename = "ListePieceJointe")]
    pub attachment: Array<Attachment>,

    #[serde(rename = "ListeThemes")]
    pub themes: Array<Theme>,

    #[serde(rename = "Matiere")]
    pub subject: ValueWrapper<ObjectReference>,

    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "PourLe")]
    pub due: Date,

    #[serde(rename = "TAFFait")]
    pub done: bool,

    #[serde(rename = "dateCDT")]
    pub notebook_date: Date,

    #[serde(rename = "descriptif")]
    pub description: Html,
}

#[derive(Debug, Deserialize)]
pub struct Attachment;

#[derive(Debug, Deserialize)]
pub struct Theme;
