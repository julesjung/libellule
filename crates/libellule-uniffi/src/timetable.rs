use libellule::model;
use time::Date;
use time::format_description::well_known::Iso8601;
use time::macros::format_description;

use crate::subject::Subject;

#[derive(Debug, uniffi::Record)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
    pub lunch_break: LunchBreak,
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
pub struct LunchBreak {
    pub start: String,
    pub end: String,
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

impl From<model::Timetable> for Timetable {
    fn from(value: model::Timetable) -> Self {
        Timetable {
            lessons: value.lessons.into_iter().map(Lesson::from).collect(),
            lunch_break: value.lunch_break.into(),
        }
    }
}

static TIME: &[time::format_description::FormatItem<'_>] =
    format_description!("[hour]:[minute]:[second]");

impl From<model::LunchBreak> for LunchBreak {
    fn from(value: model::LunchBreak) -> Self {
        LunchBreak {
            start: value.start.format(TIME).unwrap(),
            end: value.end.format(TIME).unwrap(),
        }
    }
}

impl From<model::Lesson> for Lesson {
    fn from(value: model::Lesson) -> Self {
        Lesson {
            id: value.id,
            kind: value.kind,
            start: value.start.format(TIME).unwrap(),
            end: value.end.format(TIME).unwrap(),
            subject: value.subject.into(),
            teachers: value.teachers,
            groups: value.groups.into_iter().map(Group::from).collect(),
            locations: value.locations.into_iter().map(Location::from).collect(),
            background: value.background,
        }
    }
}

impl From<model::Group> for Group {
    fn from(value: model::Group) -> Self {
        Group {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<model::Location> for Location {
    fn from(value: model::Location) -> Self {
        Location {
            id: value.id,
            name: value.name,
        }
    }
}
