use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub enum Function {
    #[serde(rename = "FonctionParametres")]
    InstanceParameters,
    Identification,
    #[serde(rename = "Authentification")]
    Authentication,
    #[serde(rename = "ParametresUtilisateur")]
    UserParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecuredData<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnsecuredData {
    #[serde(rename = "fichiers")]
    pub files: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Request<T> {
    #[serde(rename = "id")]
    pub function: Function,
    #[serde(rename = "no")]
    pub request_count: String,
    pub session: u32,
    #[serde(rename = "dataSec")]
    pub secured_data: SecuredData<T>,
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
    pub secured_data: SecuredData<T>,
    #[serde(rename = "dataNonSec")]
    pub unsecured_data: Option<UnsecuredData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Value<T> {
    #[serde(rename = "V")]
    pub value: T,
}
