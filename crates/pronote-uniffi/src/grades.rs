use pronote::models;

use crate::subject::Subject;

#[derive(Debug, uniffi::Record)]
pub struct GradesData {
    pub subjects: Vec<Subject>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, uniffi::Record)]
pub struct Assignment {
    pub id: String,
    pub label: String,
    pub grade: Grade,
    pub scale: String,
    pub coefficient: f32,
    pub date: String,
    pub subject: Subject,
    pub average: String,
    pub min_grade: String,
    pub max_grade: String,
}

#[derive(Debug, uniffi::Enum)]
pub enum Grade {
    Graded(String),
    Absent,
    Exempted,
    NotGraded,
    Unfit,
    NotSubmitted,
    AbsentZero,
    NotSubmittedZero,
}

#[derive(Debug, uniffi::Record)]
pub struct Period {
    pub id: String,
    pub name: String,
}

impl From<models::GradesData> for GradesData {
    fn from(value: models::GradesData) -> Self {
        GradesData {
            subjects: value.subjects.into_iter().map(Subject::from).collect(),
            assignments: value
                .assignments
                .into_iter()
                .map(Assignment::from)
                .collect(),
        }
    }
}

impl From<models::Assignment> for Assignment {
    fn from(value: models::Assignment) -> Self {
        Assignment {
            id: value.id,
            label: value.label,
            grade: value.grade.into(),
            scale: value.scale,
            coefficient: value.coefficient,
            date: value.date,
            subject: value.subject.into(),
            average: value.average,
            min_grade: value.min_grade,
            max_grade: value.max_grade,
        }
    }
}

impl From<models::Grade> for Grade {
    fn from(value: models::Grade) -> Self {
        match value {
            models::Grade::Graded(grade) => Grade::Graded(grade),
            models::Grade::Absent => Grade::Absent,
            models::Grade::Exempted => Grade::Exempted,
            models::Grade::NotGraded => Grade::NotGraded,
            models::Grade::Unfit => Grade::Unfit,
            models::Grade::NotSubmitted => Grade::NotSubmitted,
            models::Grade::AbsentZero => Grade::AbsentZero,
            models::Grade::NotSubmittedZero => Grade::NotSubmittedZero,
        }
    }
}

impl From<models::Period> for Period {
    fn from(value: models::Period) -> Self {
        Period {
            id: value.id,
            name: value.name,
        }
    }
}
