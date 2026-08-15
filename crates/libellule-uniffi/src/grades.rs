use libellule::model;

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

impl From<model::GradesData> for GradesData {
    fn from(value: model::GradesData) -> Self {
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

impl From<model::Assignment> for Assignment {
    fn from(value: model::Assignment) -> Self {
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

impl From<model::Grade> for Grade {
    fn from(value: model::Grade) -> Self {
        match value {
            model::Grade::Graded(grade) => Grade::Graded(grade),
            model::Grade::Absent => Grade::Absent,
            model::Grade::Exempted => Grade::Exempted,
            model::Grade::NotGraded => Grade::NotGraded,
            model::Grade::Unfit => Grade::Unfit,
            model::Grade::NotSubmitted => Grade::NotSubmitted,
            model::Grade::AbsentZero => Grade::AbsentZero,
            model::Grade::NotSubmittedZero => Grade::NotSubmittedZero,
        }
    }
}

impl From<model::Period> for Period {
    fn from(value: model::Period) -> Self {
        Period {
            id: value.id,
            name: value.name,
        }
    }
}
