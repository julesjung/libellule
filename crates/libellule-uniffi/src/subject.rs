use libellule::model;

#[derive(Debug, uniffi::Record)]
pub struct Subject {
    pub id: String,
    pub name: String,
}

impl From<model::Subject> for Subject {
    fn from(value: model::Subject) -> Self {
        Subject {
            id: value.id,
            name: value.name,
        }
    }
}
