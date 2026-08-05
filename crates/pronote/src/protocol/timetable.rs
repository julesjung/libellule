use serde::Deserialize;

use crate::protocol::Value;

#[derive(Debug, Deserialize)]
pub struct Timetable {
    #[serde(rename = "ListeCours")]
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Deserialize)]
pub struct Lesson {
    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "G")]
    pub kind: u32,

    #[serde(rename = "DateDuCours")]
    pub date: Value<String>,

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

// #[derive(Deserialize, Debug)]
// #[serde(tag = "G")]
// pub enum LessonInformation {
//     #[serde(rename = "2")]
//     Group {
//         #[serde(rename = "N")]
//         id: String,

//         #[serde(rename = "L")]
//         name: String,
//     },

//     #[serde(rename = "3")]
//     Teacher {
//         #[serde(rename = "L")]
//         name: String,
//     },

//     #[serde(rename = "16")]
//     Subject {
//         #[serde(rename = "N")]
//         id: String,

//         #[serde(rename = "L")]
//         name: String,
//     },

//     #[serde(rename = "17")]
//     Location {
//         #[serde(rename = "N")]
//         id: String,

//         #[serde(rename = "L")]
//         name: String,
//     },
// }
