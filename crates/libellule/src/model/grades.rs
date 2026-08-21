#[derive(Debug)]
pub struct GradesData {
    pub subjects: Vec<GradeSubject>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug)]
pub struct GradeSubject {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Assignment {
    pub id: String,
    pub label: String,
    pub grade: Grade,
    pub scale: String,
    pub coefficient: f32,
    pub date: String,
    pub subject: GradeSubject,
    pub average: String,
    pub min_grade: String,
    pub max_grade: String,
}

#[derive(Debug)]
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
