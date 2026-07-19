use crate::api::{function::Response, user::UserParameters};

#[derive(Debug)]
pub struct User {
    pub fullname: String,
    pub institution_name: String,
    pub class: String,
    pub profile_picture: String,
}

impl From<Response<UserParameters>> for User {
    fn from(value: Response<UserParameters>) -> User {
        let data = value.secured_data.data;
        let files = value.unsecured_data.unwrap().files;

        User {
            fullname: data.resources.name,
            institution_name: data.institution.value[0].name.clone(),
            class: data.resources.class.name,
            profile_picture: files[0].clone(),
        }
    }
}
