use libellule::model;

#[derive(Debug, uniffi::Record)]
pub struct Menu {
    pub lunch: Option<Meal>,
    pub dinner: Option<Meal>,
}

#[derive(Debug, uniffi::Record)]
pub struct Meal {
    pub starter: Option<Course>,
    pub main: Option<Course>,
    pub trimmings: Option<Course>,
    pub dairies: Option<Course>,
    pub desserts: Option<Course>,
}

#[derive(Debug, uniffi::Record)]
pub struct Course {
    pub food: Vec<Food>,
}

#[derive(Debug, uniffi::Record)]
pub struct Food {
    pub id: String,
    pub label: String,
}

impl From<model::Menu> for Menu {
    fn from(value: model::Menu) -> Self {
        Menu {
            lunch: value.lunch.map(Meal::from),
            dinner: value.dinner.map(Meal::from),
        }
    }
}

impl From<model::Meal> for Meal {
    fn from(value: model::Meal) -> Self {
        Meal {
            starter: value.starter.map(Course::from),
            main: value.main.map(Course::from),
            trimmings: value.trimmings.map(Course::from),
            dairies: value.dairies.map(Course::from),
            desserts: value.desserts.map(Course::from),
        }
    }
}

impl From<model::Course> for Course {
    fn from(value: model::Course) -> Self {
        Course {
            food: value.food.into_iter().map(Food::from).collect(),
        }
    }
}

impl From<model::Food> for Food {
    fn from(value: model::Food) -> Self {
        Food {
            id: value.id,
            label: value.label,
        }
    }
}
