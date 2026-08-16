use time::{Date, Time};

use crate::model::Subject;

#[derive(Debug)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
    pub lunch_break: LunchBreak,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct LunchBreak {
    pub start: Time,
    pub end: Time,
}

pub struct BoundaryDates {
    pub start: Date,
    pub end: Date,
}
