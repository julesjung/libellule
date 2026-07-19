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
    #[serde(rename = "DernieresNotes")]
    Grades,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecuredData<T> {
    pub data: T,
    #[serde(
        rename = "Signature",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    pub signature: Option<Signature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    #[serde(rename = "onglet")]
    tab: u32,
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
    pub fn new(
        function: Function,
        request_count: String,
        session: u32,
        tab: Option<u32>,
        data: T,
    ) -> Request<T> {
        let signature = tab.map(|tab| Signature { tab });
        let secured_data = SecuredData { data, signature };

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
