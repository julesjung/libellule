use serde::Deserialize;

use crate::protocol::ValueWrapper;

#[derive(Debug, Deserialize)]
pub struct Menu {
    #[serde(rename = "ListeJours")]
    pub days: ValueWrapper<Vec<Day>>,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    #[serde(rename = "Date")]
    pub date: ValueWrapper<String>,

    #[serde(rename = "ListeRepas")]
    pub meals: ValueWrapper<Vec<Meal>>,
}

#[derive(Debug, Deserialize)]
pub struct Meal {
    #[serde(rename = "G")]
    pub kind: u32,

    #[serde(rename = "ListePlats")]
    pub courses: ValueWrapper<Vec<Course>>,
}

#[derive(Debug, Deserialize)]
pub struct Course {
    #[serde(rename = "G")]
    pub kind: u32,

    #[serde(rename = "ListeAliments")]
    pub food: ValueWrapper<Vec<Food>>,
}

#[derive(Debug, Deserialize)]
pub struct Food {
    #[serde(rename = "N")]
    pub id: String,

    #[serde(rename = "L")]
    pub label: String,
}
