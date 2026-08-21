use serde::Deserialize;

use crate::model::Subject;
use crate::protocol::{Array, Object};

#[derive(Deserialize, Debug)]
pub struct GradesData {
    #[serde(rename = "listeServices")]
    pub subjects: Array<Subject>,
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
    pub grade: Object<Grade>,
    #[serde(rename = "bareme")]
    pub scale: Object<String>,
    pub coefficient: f32,
    #[serde(rename = "date")]
    pub date: Object<String>,
    #[serde(rename = "service")]
    pub subject: Object<Subject>,
    #[serde(rename = "moyenne")]
    pub average: Object<String>,
    #[serde(rename = "noteMin")]
    pub min_grade: Object<String>,
    #[serde(rename = "noteMax")]
    pub max_grade: Object<String>,
}

#[derive(Deserialize, Debug)]
#[serde(from = "String")]
pub enum Grade {
    Graded(String),
    Absent,
    Exempted,
    NotGraded,
    Unfit,
    NotSubmitted,
    AbsentZero,
    NotSubmittedZero,
}

impl From<String> for Grade {
    fn from(value: String) -> Self {
        match value.as_str() {
            "|1" => Grade::Absent,
            "|2" => Grade::Exempted,
            "|3" => Grade::NotGraded,
            "|4" => Grade::Unfit,
            "|5" => Grade::NotSubmitted,
            "|6" => Grade::AbsentZero,
            "|7" => Grade::NotSubmittedZero,
            _ => Grade::Graded(value),
        }
    }
}
