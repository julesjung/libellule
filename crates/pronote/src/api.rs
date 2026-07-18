use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub enum Function {
    #[serde(rename = "FonctionParametres")]
    InstanceParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecuredData<T> {
    data: T,
}

#[derive(Serialize, Debug)]
pub struct Request<T> {
    #[serde(rename = "id")]
    function: Function,
    #[serde(rename = "no")]
    request_count: String,
    session: u32,
    #[serde(rename = "dataSec")]
    secured_data: SecuredData<T>,
}

impl<T> Request<T> {
    pub fn new(function: Function, request_count: String, session: u32, data: T) -> Request<T> {
        let secured_data = SecuredData { data };

        Request {
            function,
            request_count,
            session,
            secured_data,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    #[serde(rename = "dataSec")]
    secured_data: SecuredData<T>,
}

impl<T> Response<T> {
    pub fn data(self) -> T {
        self.secured_data.data
    }
}
