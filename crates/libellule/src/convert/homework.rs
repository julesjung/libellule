use crate::convert::{array, color, date, inner_text, object};
use crate::error::ConversionError;
use crate::model;
use crate::protocol;

pub fn homework(raw: protocol::Homework) -> Result<model::Homework, ConversionError> {
    let items = array(raw.tasks)?;
    let items = items
        .into_iter()
        .map(self::item)
        .collect::<Result<_, _>>()?;

    Ok(model::Homework { items })
}

pub fn item(raw: protocol::HomeworkItem) -> Result<model::HomeworkItem, ConversionError> {
    let subject = self::subject(object(raw.subject)?);

    Ok(model::HomeworkItem {
        id: raw.id,
        subject,
        description: inner_text(raw.description),
        done: raw.done,
        creation: date(raw.given_on)?,
        due: date(raw.due)?,
        background: color(raw.background),
    })
}

pub fn subject(raw: protocol::ObjectReference) -> model::HomeworkSubject {
    model::HomeworkSubject {
        id: raw.id,
        name: raw.name,
    }
}
