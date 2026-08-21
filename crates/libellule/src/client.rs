use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;
use sha2::Digest;
use time::{Date, PlainDateTime, Time};
use tokio::sync::Mutex;
use url::Url;

use crate::convert::{grades_data, homework, timetable};
use crate::crypto::{aes_decrypt, aes_encrypt};
use crate::error::{AuthenticationError, ConversionError, Error};
use crate::instance::Instance;
use crate::model::{BoundaryDates, GradesData, Homework, Menu, Parameters, Period, Tab, Timetable};
use crate::protocol::{self, AuthenticationData, Function, IndentificationData, UserParameters};
use crate::session::{FunctionContext, Session};
use crate::time::{format_date, format_datetime};

/// An authenticated client, ready to talk to a PRONOTE instance.
#[derive(Debug)]
pub struct Client {
    instance_url: Url,
    http: reqwest::Client,
    session: Mutex<Session>,
    parameters: Parameters,
}

impl Client {
    /// Creates a client with from an `instance`.
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

        let data: IndentificationData = session.call(context, data).await?;

        let mut unencrypted_key = data.random;
        unencrypted_key.push_str(password);

        let mtp = hex::encode_upper(sha2::Sha256::digest(unencrypted_key.as_bytes()));

        let mut temporary_key = username.to_string();
        temporary_key.push_str(mtp.as_str());

        let temporary_key = md5::compute(temporary_key.as_bytes());

        let challenge =
            hex::decode(data.challenge).map_err(|_| AuthenticationError::BadChallenge)?;

        let decrypted_challenge = aes_decrypt(challenge.as_slice(), &temporary_key, &session.iv)
            .map_err(|_| AuthenticationError::InvalidCredentials)?;
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

        let data: AuthenticationData = session.call(context, data).await?;

        let encrypted_key =
            hex::decode(&data.key).map_err(|_| AuthenticationError::InvalidCredentials)?;
        let new_key = aes_decrypt(encrypted_key.as_slice(), &temporary_key, &session.iv)
            .map_err(|_| AuthenticationError::InvalidCredentials)?;
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

        let user_parameters: UserParameters = session.call(context, json!({})).await?;

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
    async fn call<S, D>(&self, function: Function, tab: Option<Tab>, data: S) -> Result<D, Error>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        let context = FunctionContext::new(&self.instance_url, &self.http, function, tab);

        self.session.lock().await.call(context, data).await
    }

    /// Returns the available grades periods.
    pub fn periods(&self) -> Vec<Period> {
        self.parameters
            .tabs
            .periods
            .get(&Tab::Grades)
            .expect("grades tab not found")
            .periods
            .clone()
    }

    /// Returns the `id` for the default [`Period`].
    pub fn default_period(&self) -> String {
        self.parameters
            .tabs
            .periods
            .get(&Tab::Grades)
            .expect("grades tab not found")
            .default
            .clone()
    }

    /// Fetches the Grades for a specific `period`.
    pub async fn grades(&self, period: &Period) -> Result<GradesData, Error> {
        let data = json!({
            "Periode": period
        });

        let raw: protocol::GradesData =
            self.call(Function::Grades, Some(Tab::Grades), data).await?;

        let model = grades_data(raw)?;

        Ok(model)
    }

    /// Returns the date range allowed for the timetable.
    pub fn boundary_dates(&self) -> BoundaryDates {
        BoundaryDates {
            start: self.parameters.instance.first_day,
            end: self.parameters.instance.last_day,
        }
    }

    /// Fetches the timetable for a specific `date`.
    pub async fn timetable(&self, date: Date) -> Result<Timetable, Error> {
        let user = &self.parameters.user;

        let user = json!({
            "L": user.fullname,
            "N": user.id,
            "G": user.kind
        });

        let date = format_datetime(PlainDateTime::new(date, Time::MIDNIGHT));

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

        let raw: protocol::Timetable = self
            .call(Function::Timetable, Some(Tab::Timetable), data)
            .await?;

        let model = timetable(raw, &self.parameters)?;

        Ok(model)
    }

    /// Fetches the menu for a specific `date`.
    pub async fn menu(&self, date: Date) -> Result<Menu, Error> {
        let date = format_date(date);

        let data = json!({
            "date": {
                "_T": 7,
                "V": &date
            }
        });

        let data: protocol::Menu = self.call(Function::Menu, Some(Tab::Menu), data).await?;

        let day = data
            .days
            .value
            .into_iter()
            .find(|day| day.date.value == date);

        let menu = match day {
            Some(day) => day.try_into().map_err(|_| ConversionError::Parse)?,
            None => Menu {
                lunch: None,
                dinner: None,
            },
        };

        Ok(menu)
    }

    /// Returns homework for the week containing `date`.
    pub async fn homework(&self, date: Date) -> Result<Homework, Error> {
        // TODO: check that date is within range
        let week = (date - self.parameters.instance.first_monday).whole_weeks();
        let week = format!("[{week}]");

        let data = json!({
            "domaine": {
                "_T": 8,
                "V": week
            }
        });

        let raw: protocol::Homework = self
            .call(Function::Homework, Some(Tab::Homework), data)
            .await?;

        let model = homework(raw)?;

        Ok(model)
    }
}
