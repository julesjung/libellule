use std::ops::Add;
use std::time::Duration;

use time::PlainDateTime;

use crate::error::Error;
use crate::models::Subject;
use crate::protocol;
use crate::time::parse_datetime;

#[derive(Debug)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
}

#[derive(Debug)]
pub struct Lesson {
    pub id: String,
    pub kind: u32,
    pub start: PlainDateTime,
    pub end: PlainDateTime,
    pub subject: Subject,
    pub teachers: Vec<String>,
    pub groups: Vec<Group>,
    pub locations: Vec<Location>,
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

impl TryFrom<protocol::Timetable> for Timetable {
    type Error = Error;

    fn try_from(value: protocol::Timetable) -> Result<Self, Self::Error> {
        let mut lessons: Vec<Lesson> = value
            .lessons
            .into_iter()
            .map(Lesson::try_from)
            .collect::<Result<_, _>>()?;

        lessons.sort_by_key(|lesson| lesson.start);

        Ok(Timetable { lessons })
    }
}

impl TryFrom<protocol::Lesson> for Lesson {
    type Error = Error;

    fn try_from(value: protocol::Lesson) -> Result<Self, Error> {
        let start = parse_datetime(&value.date.value)?;
        let end = start.add(Duration::from_hours(1));

        let mut teachers = Vec::new();
        let mut subject = None;
        let mut locations = Vec::new();
        let mut groups = Vec::new();

        for information in value.information.value.into_iter() {
            match information.kind {
                2 => groups.push(Group {
                    id: information.id.unwrap(),
                    name: information.label,
                }),
                3 => teachers.push(information.label),
                16 => {
                    subject = Some(Subject {
                        id: information.id.unwrap(),
                        name: information.label,
                    })
                }
                17 => locations.push(Location {
                    id: information.id.unwrap(),
                    name: information.label,
                }),
                other => return Err(Error::UnknownLessonInformationKind { lesson_kind: other }),
            };
        }

        Ok(Lesson {
            id: value.id,
            kind: value.kind,
            start,
            end,
            teachers,
            subject: subject.unwrap(),
            locations,
            groups,
        })
    }
}
