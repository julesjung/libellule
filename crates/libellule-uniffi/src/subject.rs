use libellule::model::Subject;

#[uniffi::remote(Record)]
pub struct Subject {
    pub id: String,
    pub name: String,
}
