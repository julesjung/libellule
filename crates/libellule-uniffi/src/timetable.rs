use libellule::models;
use time::{Date, format_description::well_known::Iso8601, macros::format_description};

use crate::subject::Subject;

#[derive(Debug, uniffi::Record)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, uniffi::Record)]
pub struct Lesson {
    pub id: String,
    pub kind: u32,
    pub start: String,
    pub end: String,
    pub subject: Subject,
    pub teachers: Vec<String>,
    pub groups: Vec<Group>,
    pub locations: Vec<Location>,
    pub background: String,
}

#[derive(Debug, uniffi::Record)]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(Debug, uniffi::Record)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Debug, uniffi::Record)]
pub struct BoundaryDates {
    first: String,
    second: String,
}

impl From<(Date, Date)> for BoundaryDates {
    fn from(value: (Date, Date)) -> Self {
        BoundaryDates {
            first: value.0.format(&Iso8601::DATE).unwrap(),
            second: value.1.format(&Iso8601::DATE).unwrap(),
        }
    }
}

impl From<models::Timetable> for Timetable {
    fn from(value: models::Timetable) -> Self {
        Timetable {
            lessons: value.lessons.into_iter().map(Lesson::from).collect(),
        }
    }
}

static DATE_TIME: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

impl From<models::Lesson> for Lesson {
    fn from(value: models::Lesson) -> Self {
        Lesson {
            id: value.id,
            kind: value.kind,
            start: value.start.format(DATE_TIME).unwrap(),
            end: value.end.format(DATE_TIME).unwrap(),
            subject: value.subject.into(),
            teachers: value.teachers,
            groups: value.groups.into_iter().map(Group::from).collect(),
            locations: value.locations.into_iter().map(Location::from).collect(),
            background: value.background,
        }
    }
}

impl From<models::Group> for Group {
    fn from(value: models::Group) -> Self {
        Group {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<models::Location> for Location {
    fn from(value: models::Location) -> Self {
        Location {
            id: value.id,
            name: value.name,
        }
    }
}
