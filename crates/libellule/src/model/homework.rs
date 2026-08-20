use time::Date;

#[derive(Debug)]
pub struct Homework {
    pub items: Vec<HomeworkItem>,
}

#[derive(Debug)]
pub struct HomeworkItem {
    pub id: String,
    pub subject: HomeworkSubject,
    pub description: String,
    pub done: bool,
    pub creation: Date,
    pub due: Date,
    pub background: String,
}

#[derive(Debug)]
pub struct HomeworkSubject {
    pub id: String,
    pub name: String,
}
