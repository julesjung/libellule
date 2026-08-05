use pronote::models;

#[derive(Debug, uniffi::Record)]
pub struct Subject {
    pub id: String,
    pub name: String,
}

impl From<models::Subject> for Subject {
    fn from(value: models::Subject) -> Self {
        Subject {
            id: value.id,
            name: value.name,
        }
    }
}
