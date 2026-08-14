use serde::Deserialize;

use crate::protocol::Value;

#[derive(Debug, Deserialize)]
pub struct Menu {
    #[serde(rename = "ListeJours")]
    pub days: Value<Vec<Day>>,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    #[serde(rename = "Date")]
    pub date: Value<String>,

    #[serde(rename = "ListeRepas")]
    pub meals: Value<Vec<Meal>>,
}

#[derive(Debug, Deserialize)]
pub struct Meal {
    #[serde(rename = "G")]
    pub kind: u32,

    #[serde(rename = "ListePlats")]
    pub courses: Value<Vec<Course>>,
}

#[derive(Debug, Deserialize)]
pub struct Course {
    #[serde(rename = "G")]
    pub kind: u32,

    #[serde(rename = "ListeAliments")]
    pub food: Value<Vec<Food>>,
}

#[derive(Debug, Deserialize)]
pub struct Food {
    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "L")]
    pub label: String,
}
