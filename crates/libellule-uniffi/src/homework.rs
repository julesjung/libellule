use libellule::model::{Homework, HomeworkItem, HomeworkSubject};
use time::Date;

#[uniffi::remote(Record)]
struct Homework {
    items: Vec<HomeworkItem>,
}

#[uniffi::remote(Record)]
struct HomeworkItem {
    id: String,
    subject: HomeworkSubject,
    description: String,
    done: bool,
    creation: Date,
    due: Date,
    background: String,
}

#[uniffi::remote(Record)]
pub struct HomeworkSubject {
    pub id: String,
    pub name: String,
}
