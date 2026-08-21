use crate::{error::ConversionError, model, protocol};

pub fn grades_data(raw: protocol::GradesData) -> Result<model::GradesData, ConversionError> {
    let subjects = raw
        .subjects
        .0
        .into_iter()
        .map(self::grade_subject)
        .collect();

    let assignments = raw
        .assignments
        .0
        .into_iter()
        .map(self::assignment)
        .collect::<Result<_, _>>()?;

    Ok(model::GradesData {
        subjects,
        assignments,
    })
}

pub fn grade_subject(raw: protocol::GradeSubject) -> model::GradeSubject {
    model::GradeSubject {
        id: raw.id,
        name: raw.name,
    }
}

pub fn assignment(raw: protocol::Assignment) -> Result<model::Assignment, ConversionError> {
    Ok(model::Assignment {
        id: raw.id,
        label: raw.label,
        grade: grade(raw.grade.0),
        scale: raw.scale.0,
        coefficient: raw.coefficient,
        date: raw.date.0,
        subject: grade_subject(raw.subject.0),
        average: raw.average.0,
        min_grade: raw.min_grade.0,
        max_grade: raw.max_grade.0,
    })
}

pub fn grade(raw: String) -> model::Grade {
    match raw.as_str() {
        "|1" => model::Grade::Absent,
        "|2" => model::Grade::Exempted,
        "|3" => model::Grade::NotGraded,
        "|4" => model::Grade::Unfit,
        "|5" => model::Grade::NotSubmitted,
        "|6" => model::Grade::AbsentZero,
        "|7" => model::Grade::NotSubmittedZero,
        _ => model::Grade::Graded(raw),
    }
}
