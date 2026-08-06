use serde::Deserialize;
use serde_with::serde_as;

use crate::models::Subject;
use crate::protocol::FromValue;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct GradesData {
    #[serde(rename = "listeServices")]
    #[serde_as(as = "FromValue")]
    pub subjects: Vec<Subject>,
    #[serde(rename = "listeDevoirs")]
    #[serde_as(as = "FromValue")]
    pub assignments: Vec<Assignment>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Assignment {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "commentaire")]
    pub label: String,
    #[serde(rename = "note")]
    #[serde_as(as = "FromValue")]
    pub grade: Grade,
    #[serde(rename = "bareme")]
    #[serde_as(as = "FromValue")]
    pub scale: String,
    pub coefficient: f32,
    #[serde(rename = "date")]
    #[serde_as(as = "FromValue")]
    pub date: String,
    #[serde(rename = "service")]
    #[serde_as(as = "FromValue")]
    pub subject: Subject,
    #[serde(rename = "moyenne")]
    #[serde_as(as = "FromValue")]
    pub average: String,
    #[serde(rename = "noteMin")]
    #[serde_as(as = "FromValue")]
    pub min_grade: String,
    #[serde(rename = "noteMax")]
    #[serde_as(as = "FromValue")]
    pub max_grade: String,
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
