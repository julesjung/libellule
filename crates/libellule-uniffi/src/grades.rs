use libellule::model::{Assignment, Grade, GradesData, Period, Subject};

#[uniffi::remote(Record)]
pub struct GradesData {
    pub subjects: Vec<Subject>,
    pub assignments: Vec<Assignment>,
}

#[uniffi::remote(Record)]
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

#[uniffi::remote(Enum)]
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

#[uniffi::remote(Record)]
pub struct Period {
    pub id: String,
    pub name: String,
}
