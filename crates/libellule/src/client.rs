use serde_json::json;
use sha2::Digest;
use time::{Date, PlainDateTime, Time};
use tokio::sync::Mutex;
use url::Url;

use crate::convert::TryModelize;
use crate::crypto::{aes_decrypt, aes_encrypt};
use crate::error::Error;
use crate::instance::Instance;
use crate::model::{ConversionError, GradesData, Menu, Parameters, Period, Tab, Timetable};
use crate::protocol;
use crate::protocol::{
    AuthenticationData, Empty, Function, IndentificationData, Response, UserParameters,
};
use crate::session::{FunctionContext, Session};
use crate::time::{format_date, format_datetime};

#[derive(Debug)]
pub struct Client {
    instance_url: Url,
    http: reqwest::Client,
    session: Mutex<Session>,
    parameters: Parameters,
}

impl Client {
    pub async fn login(
        instance: &Instance,
        username: &str,
        password: &str,
    ) -> Result<Client, Error> {
        let mut session = instance.session.clone();

        let context = FunctionContext::new(
            &instance.base_url,
            &instance.http,
            Function::Identification,
            None,
        );

        let data = json!({
            "genreConnexion": 0,
            "genreEspace": 3,
            "identifiant": username,
            "pourENT": false,
            "enConnexionAuto": false,
            "demandeConnexionAuto": false,
            "enConnexionAppliMobile": false,
            "demandeConnexionAppliMobile": false,
            "demandeConnexionAppliMobileJeton": false,
            "uuidAppliMobile": "",
            "loginTokenSAV": "",
            "informationsAppareil": null
        });

        let response: Response<IndentificationData> = session.call(context, data).await?;
        let data = response.secured_data.data;

        let mut unencrypted_key = data.random;
        unencrypted_key.push_str(password);

        let mtp = hex::encode_upper(sha2::Sha256::digest(unencrypted_key.as_bytes()));

        let mut temporary_key = username.to_string();
        temporary_key.push_str(mtp.as_str());

        let temporary_key = md5::compute(temporary_key.as_bytes());

        let challenge = hex::decode(data.challenge)?;

        let decrypted_challenge = aes_decrypt(challenge.as_slice(), &temporary_key, &session.iv)?;
        let decrypted_challenge = String::from_utf8(decrypted_challenge).unwrap();

        let solution: String = decrypted_challenge
            .chars()
            .enumerate()
            .filter_map(|(index, character)| {
                if index % 2 == 0 {
                    Some(character)
                } else {
                    None
                }
            })
            .collect();

        let encrypted_solution = aes_encrypt(solution.as_bytes(), &temporary_key, &session.iv);
        let encrypted_solution = hex::encode(encrypted_solution);

        let context = FunctionContext::new(
            &instance.base_url,
            &instance.http,
            Function::Authentication,
            None,
        );

        let data = json!({
            "connexion": 0,
            "challenge": encrypted_solution,
            "espace": 3
        });

        let response: Response<AuthenticationData> = session.call(context, data).await?;
        let data = response.secured_data.data;

        let encrypted_key = hex::decode(&data.key)?;
        let new_key = aes_decrypt(encrypted_key.as_slice(), &temporary_key, &session.iv)?;
        let new_key: Vec<u8> = String::from_utf8(new_key)
            .unwrap()
            .split(',')
            .map(|byte| byte.parse::<u8>().unwrap())
            .collect();

        session.key = *md5::compute(new_key);

        let context = FunctionContext::new(
            &instance.base_url,
            &instance.http,
            Function::UserParameters,
            None,
        );

        let response: Response<UserParameters> = session.call(context, Empty::new()).await?;
        let user_parameters = response.secured_data.data;

        let parameters = Parameters::try_from((instance.parameters.clone(), user_parameters))?;

        Ok(Client {
            instance_url: instance.base_url.clone(),
            http: instance.http.clone(),
            parameters,
            session: Mutex::new(session),
        })
    }
}

impl Client {
    pub fn get_periods(&self) -> Vec<Period> {
        self.parameters
            .tabs
            .periods
            .get(&Tab::Grades)
            .expect("grades tab not found")
            .periods
            .clone()
    }

    pub fn get_default_period(&self) -> String {
        self.parameters
            .tabs
            .periods
            .get(&Tab::Grades)
            .expect("grades tab not found")
            .default
            .clone()
    }

    pub async fn get_grades(&self, period: &Period) -> Result<GradesData, Error> {
        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::Grades,
            Some(Tab::Grades),
        );

        let data = json!({
            "Periode": period
        });

        let response: Response<GradesData> = self.session.lock().await.call(context, data).await?;

        Ok(response.secured_data.data)
    }

    pub fn boundary_dates(&self) -> (Date, Date) {
        (
            self.parameters.instance.first_day,
            self.parameters.instance.last_day,
        )
    }

    pub async fn timetable(&self, date: Date) -> Result<Timetable, Error> {
        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::Timetable,
            Some(Tab::Timetable),
        );

        let user = &self.parameters.user;

        let user = json!({
            "L": user.fullname,
            "N": user.id,
            "G": user.kind
        });

        let date = format_datetime(PlainDateTime::new(date, Time::MIDNIGHT))?;

        let data = json!({
            "avecAbsencesEleve": true,
            "avecAbsencesRessource": true,
            "avecConseilDeClasse": true,
            "avecCoursSortiePeda": true,
            "avecDisponibilites": true,
            "avecInfosPrefsGrille": true,
            "avecRetenuesEleve": true,
            "DateDebut": {
                "_T": 7,
                "V": date
            },
            "DateDebut": {
                "_T": 7,
                "V": date
            },
            "estEDTAnnuel": false,
            "estEDTPermanence": false,
            "ressource": user,
            "Ressource": user
        });

        let response: Response<protocol::Timetable> =
            self.session.lock().await.call(context, data).await?;

        response.secured_data.data.try_modelize(&self.parameters)
    }

    pub async fn menu(&self, date: Date) -> Result<Menu, Error> {
        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::Menu,
            Some(Tab::Menu),
        );

        let date = format_date(date);

        let data = json!({
            "date": {
                "_T": 7,
                "V": &date
            }
        });

        let response: Response<protocol::Menu> =
            self.session.lock().await.call(context, data).await?;

        let day = response
            .secured_data
            .data
            .days
            .value
            .into_iter()
            .find(|day| day.date.value == date);

        let menu = match day {
            Some(day) => day.try_into().map_err(|err| ConversionError::Menu(err))?,
            None => Menu {
                lunch: None,
                dinner: None,
            },
        };

        Ok(menu)
    }
}
