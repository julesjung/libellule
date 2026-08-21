use crate::convert::{TryModelize, TryModelizeWith};
use crate::error::{ConversionError, Error};
use crate::model::Parameters;
use crate::model::{self, LunchBreak};
use crate::protocol;

impl TryModelize<model::Timetable> for protocol::Timetable {
    type Error = Error;

    fn try_modelize(self, parameters: &Parameters) -> Result<model::Timetable, Self::Error> {
        let mut lessons: Vec<model::Lesson> = self
            .lessons
            .into_iter()
            .map(|lesson| lesson.try_modelize_with(parameters, &self.start_place))
            .collect::<Result<_, _>>()?;

        lessons.sort_by_key(|lesson| lesson.start);

        let lunch_break_start = parameters.place_to_time(self.lunch_break_start - self.start_place);
        let lunch_break_end = parameters.place_to_time(self.lunch_break_end - self.start_place);

        let lunch_break = LunchBreak {
            start: lunch_break_start,
            end: lunch_break_end,
        };

        Ok(model::Timetable {
            lessons,
            lunch_break,
        })
    }
}

impl TryModelizeWith<model::Lesson> for protocol::Lesson {
    type With = u32;
    type Error = Error;

    fn try_modelize_with(
        self,
        parameters: &Parameters,
        with: &Self::With,
    ) -> Result<model::Lesson, Self::Error> {
        let start = parameters.place_to_time(self.start - with);
        let end = start + parameters.instance.place_duration * self.length;

        let mut teachers = Vec::new();
        let mut subject = None;
        let mut locations = Vec::new();
        let mut groups = Vec::new();

        for information in self.information.value.into_iter() {
            match information.kind {
                2 => groups.push(model::Group {
                    id: information.id.unwrap(),
                    name: information.label,
                }),
                3 => teachers.push(information.label),
                16 => {
                    subject = Some(model::TimetableSubject {
                        id: information.id.unwrap(),
                        name: information.label,
                    })
                }
                17 => locations.push(model::Location {
                    id: information.id.unwrap(),
                    name: information.label,
                }),
                _ => return Err(ConversionError::Parse.into()),
            };
        }

        Ok(model::Lesson {
            id: self.id,
            kind: self.kind,
            start,
            end,
            teachers,
            subject: subject.unwrap(),
            locations,
            groups,
            background: self.background_color,
        })
    }
}
