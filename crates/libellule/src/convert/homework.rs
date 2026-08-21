use super::value::{date, inner_text};

use crate::error::ConversionError;
use crate::model;
use crate::protocol;

pub(crate) fn homework(raw: protocol::Homework) -> Result<model::Homework, ConversionError> {
    let items = raw.tasks.0;
    let items = items
        .into_iter()
        .map(self::item)
        .collect::<Result<_, _>>()?;

    Ok(model::Homework { items })
}

fn item(raw: protocol::HomeworkItem) -> Result<model::HomeworkItem, ConversionError> {
    let subject = self::subject(raw.subject.0);

    Ok(model::HomeworkItem {
        id: raw.id,
        subject,
        description: inner_text(raw.description),
        done: raw.done,
        creation: date(raw.given_on)?,
        due: date(raw.due)?,
        background: raw.background.0,
    })
}

fn subject(raw: protocol::ObjectReference) -> model::HomeworkSubject {
    model::HomeworkSubject {
        id: raw.id,
        name: raw.name,
    }
}
