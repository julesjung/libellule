use serde::{Deserialize, Serialize};

use crate::protocol::{Array, Date, Grade, Object};

#[derive(Debug, Deserialize)]
pub struct GradesData {
    #[serde(rename = "listeServices")]
    pub subjects: Array<GradeSubject>,
    #[serde(rename = "listeDevoirs")]
    pub assignments: Array<Assignment>,
}

#[derive(Deserialize, Debug)]
pub struct Assignment {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "commentaire")]
    pub label: String,
    #[serde(rename = "note")]
    pub grade: Grade,
    #[serde(rename = "bareme")]
    pub scale: Grade,
    pub coefficient: f32,
    pub date: Date,
    #[serde(rename = "service")]
    pub subject: Object<GradeSubject>,
    #[serde(rename = "moyenne")]
    pub average: Grade,
    #[serde(rename = "noteMin")]
    pub min_grade: Grade,
    #[serde(rename = "noteMax")]
    pub max_grade: Grade,
}

#[derive(Debug, Deserialize)]
pub struct GradeSubject {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "L")]
    pub name: String,
}
