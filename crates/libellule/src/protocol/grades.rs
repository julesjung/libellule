use serde::Deserialize;

use crate::protocol::{Array, Date, Grade, Object};

#[derive(Debug, Deserialize)]
pub(crate) struct GradesData {
    #[serde(rename = "listeServices")]
    pub(crate) subjects: Array<GradeSubject>,

    #[serde(rename = "listeDevoirs")]
    pub(crate) assignments: Array<Assignment>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Assignment {
    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "commentaire")]
    pub(crate) label: String,

    #[serde(rename = "note")]
    pub(crate) grade: Grade,

    #[serde(rename = "bareme")]
    pub(crate) scale: Grade,

    pub(crate) coefficient: f32,

    pub(crate) date: Date,

    #[serde(rename = "service")]
    pub(crate) subject: Object<GradeSubject>,

    #[serde(rename = "moyenne")]
    pub(crate) average: Grade,

    #[serde(rename = "noteMin")]
    pub(crate) min_grade: Grade,

    #[serde(rename = "noteMax")]
    pub(crate) max_grade: Grade,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GradeSubject {
    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "L")]
    pub(crate) name: String,
}
