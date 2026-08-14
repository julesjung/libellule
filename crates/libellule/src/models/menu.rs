use crate::protocol;

#[derive(Debug, thiserror::Error)]
pub enum MenuError {
    #[error("unknown meal kind `{kind}`")]
    UnknownMeal { kind: u32 },
    #[error("missing starter course")]
    MissingStarterCourse,
    #[error("missing main course")]
    MissingMainCourse,
    #[error("missing trimmings")]
    MissingTrimmings,
    #[error("missing dairies")]
    MissingDairies,
    #[error("missing dessert")]
    MissingDessert,
    #[error("unknown course kind `{kind}`")]
    UnknownCourse { kind: u32 },
}

#[derive(Debug)]
pub struct Menu {
    pub lunch: Option<Meal>,
    pub dinner: Option<Meal>,
}

#[derive(Debug)]
pub struct Meal {
    pub starter: Course,
    pub main: Course,
    pub trimmings: Course,
    pub dairies: Course,
    pub desserts: Course,
}

#[derive(Debug)]
pub struct Course(Vec<Food>);

#[derive(Debug)]
pub struct Food {
    id: String,
    label: String,
}

impl TryFrom<protocol::Menu> for Vec<Menu> {
    type Error = MenuError;

    fn try_from(value: protocol::Menu) -> Result<Self, Self::Error> {
        value.days.value.into_iter().map(Menu::try_from).collect()
    }
}

impl TryFrom<protocol::Day> for Menu {
    type Error = MenuError;

    fn try_from(value: protocol::Day) -> Result<Self, MenuError> {
        let mut lunch = None;
        let mut dinner = None;

        for meal in value.meals.value.into_iter() {
            let kind = meal.kind;
            let meal = meal.try_into()?;

            match kind {
                0 => lunch = Some(meal),
                1 => dinner = Some(meal),
                other => return Err(MenuError::UnknownMeal { kind: other }),
            }
        }

        Ok(Menu { lunch, dinner })
    }
}

impl TryFrom<protocol::Meal> for Meal {
    type Error = MenuError;

    fn try_from(value: protocol::Meal) -> Result<Self, MenuError> {
        let mut starter = None;
        let mut main = None;
        let mut trimmings = None;
        let mut dairies = None;
        let mut desserts = None;

        for course in value.courses.value.into_iter() {
            let kind = course.kind;
            let course: Course = course.into();

            match kind {
                0 => starter = Some(course),
                1 => main = Some(course),
                2 => trimmings = Some(course),
                5 => dairies = Some(course),
                4 => desserts = Some(course),
                other => return Err(MenuError::UnknownCourse { kind: other }),
            }
        }

        Ok(Meal {
            starter: starter.ok_or(MenuError::MissingStarterCourse)?,
            main: main.ok_or(MenuError::MissingMainCourse)?,
            trimmings: trimmings.ok_or(MenuError::MissingTrimmings)?,
            dairies: dairies.ok_or(MenuError::MissingDairies)?,
            desserts: desserts.ok_or(MenuError::MissingDessert)?,
        })
    }
}

impl From<protocol::Course> for Course {
    fn from(value: protocol::Course) -> Self {
        let food = value.food.value.into_iter().map(Food::from).collect();

        Course(food)
    }
}

impl From<protocol::Food> for Food {
    fn from(value: protocol::Food) -> Self {
        Food {
            id: value.id,
            label: value.label,
        }
    }
}
