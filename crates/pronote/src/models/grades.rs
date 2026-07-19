use crate::api;

#[derive(Debug)]
pub struct GradesData {
    pub subjects: Vec<Subject>,
    pub assignments: Vec<Assignment>,
}

impl From<api::GradesData> for GradesData {
    fn from(value: api::GradesData) -> GradesData {
        GradesData {
            subjects: value
                .subjects
                .value
                .into_iter()
                .map(|subject| subject.into())
                .collect(),
            assignments: value
                .assignments
                .value
                .into_iter()
                .map(|subject| subject.into())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Subject {
    pub id: String,
    pub name: String,
}

impl From<api::Subject> for Subject {
    fn from(value: api::Subject) -> Subject {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug)]
pub struct Assignment {
    pub id: String,
    pub label: String,
    pub grade: Option<f32>,
    pub scale: f32,
    pub coefficient: f32,
    pub date: String,
    pub subject: Subject,
    pub average: f32,
    pub min_grade: f32,
    pub max_grade: f32,
}

impl From<api::Assignment> for Assignment {
    fn from(value: api::Assignment) -> Assignment {
        dbg!(&value.grade);
        Assignment {
            id: value.id,
            label: value.label,
            grade: value.grade.value.replace(',', ".").parse().ok(),
            scale: value.scale.value.replace(',', ".").parse().unwrap(),
            coefficient: value.coefficient,
            date: value.date.value,
            subject: value.subject.value.into(),
            average: value.average.value.replace(',', ".").parse().unwrap(),
            min_grade: value.min_grade.value.replace(',', ".").parse().unwrap(),
            max_grade: value.max_grade.value.replace(',', ".").parse().unwrap(),
        }
    }
}
