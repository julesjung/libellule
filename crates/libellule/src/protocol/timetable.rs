use serde::Deserialize;

use crate::protocol::Value;

#[derive(Debug, Deserialize)]
pub struct Timetable {
    #[serde(rename = "ListeCours")]
    pub lessons: Vec<Lesson>,

    #[serde(rename = "premierePlaceHebdoDuJour")]
    pub start_place: u32,

    #[serde(rename = "debutDemiPensionHebdo")]
    pub lunch_break_start: u32,

    #[serde(rename = "finDemiPensionHebdo")]
    pub lunch_break_end: u32,
}

#[derive(Debug, Deserialize)]
pub struct Lesson {
    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "G")]
    pub kind: u32,

    #[serde(rename = "place")]
    pub start: u32,

    #[serde(rename = "duree")]
    pub length: u32,

    #[serde(rename = "estAnnule", default)]
    pub cancelled: bool,

    #[serde(rename = "ListeContenus")]
    pub information: Value<Vec<LessonInformation>>,

    #[serde(rename = "CouleurFond")]
    pub background_color: String,
}

#[derive(Deserialize, Debug)]
pub struct LessonInformation {
    #[serde(rename = "N")]
    pub id: Option<String>,

    #[serde(rename = "L")]
    pub label: String,

    #[serde(rename = "G")]
    pub kind: u32,
}
