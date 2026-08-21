use serde::Deserialize;

use crate::protocol::ValueWrapper;

#[derive(Debug, Deserialize)]
pub(crate) struct Menu {
    #[serde(rename = "ListeJours")]
    pub(crate) days: ValueWrapper<Vec<Day>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Day {
    #[serde(rename = "Date")]
    pub(crate) date: ValueWrapper<String>,

    #[serde(rename = "ListeRepas")]
    pub(crate) meals: ValueWrapper<Vec<Meal>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Meal {
    #[serde(rename = "G")]
    pub(crate) kind: u32,

    #[serde(rename = "ListePlats")]
    pub(crate) courses: ValueWrapper<Vec<Course>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Course {
    #[serde(rename = "G")]
    pub(crate) kind: u32,

    #[serde(rename = "ListeAliments")]
    pub(crate) food: ValueWrapper<Vec<Food>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Food {
    #[serde(rename = "N")]
    pub(crate) id: String,

    #[serde(rename = "L")]
    pub(crate) label: String,
}
