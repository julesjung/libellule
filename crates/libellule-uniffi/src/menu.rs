use libellule::model::{Course, Food, Meal, Menu};

#[uniffi::remote(Record)]
pub struct Menu {
    pub lunch: Option<Meal>,
    pub dinner: Option<Meal>,
}

#[uniffi::remote(Record)]
pub struct Meal {
    pub starter: Option<Course>,
    pub main: Option<Course>,
    pub trimmings: Option<Course>,
    pub dairies: Option<Course>,
    pub desserts: Option<Course>,
}

#[uniffi::remote(Record)]
pub struct Course {
    pub food: Vec<Food>,
}

#[uniffi::remote(Record)]
pub struct Food {
    pub id: String,
    pub label: String,
}
