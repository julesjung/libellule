use crate::error::ConversionError;
use crate::model;
use crate::protocol;

pub(crate) fn timetable(
    raw: protocol::Timetable,
    parameters: &model::Parameters,
) -> Result<model::Timetable, ConversionError> {
    let mut lessons: Vec<model::Lesson> = raw
        .lessons
        .into_iter()
        .map(|lesson| self::lesson(lesson, parameters, raw.start_place))
        .collect::<Result<_, _>>()?;

    lessons.sort_by_key(|lesson| lesson.start);

    let lunch_break_start = parameters.place_to_time(raw.lunch_break_start - raw.start_place);
    let lunch_break_end = parameters.place_to_time(raw.lunch_break_end - raw.start_place);

    let lunch_break = model::LunchBreak {
        start: lunch_break_start,
        end: lunch_break_end,
    };

    Ok(model::Timetable {
        lessons,
        lunch_break,
    })
}

fn lesson(
    raw: protocol::Lesson,
    parameters: &model::Parameters,
    start: u32,
) -> Result<model::Lesson, ConversionError> {
    let start = parameters.place_to_time(raw.start - start);
    let end = start + parameters.instance.place_duration * raw.length;

    let mut teachers = Vec::new();
    let mut subject = None;
    let mut locations = Vec::new();
    let mut groups = Vec::new();

    for information in raw.information.value.into_iter() {
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
            _ => return Err(ConversionError::Parse),
        };
    }

    Ok(model::Lesson {
        id: raw.id,
        kind: raw.kind,
        start,
        end,
        teachers,
        subject: subject.unwrap(),
        locations,
        groups,
        background: raw.background_color,
    })
}
