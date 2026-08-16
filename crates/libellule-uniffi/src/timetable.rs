use libellule::model::{BoundaryDates, Group, Lesson, Location, LunchBreak, Subject, Timetable};
use time::{Date, Time};

#[uniffi::remote(Record)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
    pub lunch_break: LunchBreak,
}

#[uniffi::remote(Record)]
pub struct Lesson {
    pub id: String,
    pub kind: u32,
    pub start: Time,
    pub end: Time,
    pub subject: Subject,
    pub teachers: Vec<String>,
    pub groups: Vec<Group>,
    pub locations: Vec<Location>,
    pub background: String,
}

#[uniffi::remote(Record)]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[uniffi::remote(Record)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[uniffi::remote(Record)]
pub struct LunchBreak {
    pub start: Time,
    pub end: Time,
}

#[uniffi::remote(Record)]
pub struct BoundaryDates {
    start: Date,
    end: Date,
}
