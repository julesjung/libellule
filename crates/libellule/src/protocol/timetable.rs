use serde::Deserialize;

use crate::protocol::ValueWrapper;

#[derive(Debug, Deserialize)]
pub(crate) struct Timetable {
    #[serde(rename = "ListeCours")]
    pub(crate) lessons: Vec<Lesson>,

    #[serde(rename = "premierePlaceHebdoDuJour")]
    pub(crate) start_place: u32,

    #[serde(rename = "debutDemiPensionHebdo")]
    pub(crate) lunch_break_start: u32,

    #[serde(rename = "finDemiPensionHebdo")]
    pub(crate) lunch_break_end: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Lesson {
    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "G")]
    pub(crate) kind: u32,

    #[serde(rename = "place")]
    pub(crate) start: u32,

    #[serde(rename = "duree")]
    pub(crate) length: u32,

    #[serde(rename = "estAnnule", default)]
    pub(crate) _cancelled: bool,

    #[serde(rename = "ListeContenus")]
    pub(crate) information: ValueWrapper<Vec<LessonInformation>>,

    #[serde(rename = "CouleurFond")]
    pub(crate) background_color: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LessonInformation {
    #[serde(rename = "N")]
    pub(crate) id: Option<String>,

    #[serde(rename = "L")]
    pub(crate) label: String,

    #[serde(rename = "G")]
    pub(crate) kind: u32,
}
