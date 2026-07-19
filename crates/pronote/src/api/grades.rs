use serde::Deserialize;

use crate::api::Value;

#[derive(Deserialize, Debug)]
pub struct GradesData {
    #[serde(rename = "listeServices")]
    pub subjects: Value<Vec<Subject>>,
    #[serde(rename = "listeDevoirs")]
    pub assignments: Value<Vec<Assignment>>,
}

#[derive(Deserialize, Debug)]
pub struct Subject {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "L")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Assignment {
    #[serde(rename = "N")]
    pub id: String,
    #[serde(rename = "commentaire")]
    pub label: String,
    #[serde(rename = "note")]
    pub grade: Value<String>,
    #[serde(rename = "bareme")]
    pub scale: Value<String>,
    pub coefficient: f32,
    #[serde(rename = "date")]
    pub date: Value<String>,
    #[serde(rename = "service")]
    pub subject: Value<Subject>,
    #[serde(rename = "moyenne")]
    pub average: Value<String>,
    #[serde(rename = "noteMin")]
    pub min_grade: Value<String>,
    #[serde(rename = "noteMax")]
    pub max_grade: Value<String>,
}
