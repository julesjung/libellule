use libellule::models;
use time::Date;
use time::format_description::well_known::Iso8601;
use tokio::sync::Mutex;

use crate::error::LibelluleError;
use crate::grades::{GradesData, Period};
use crate::instance::Instance;
use crate::timetable::Timetable;

#[derive(uniffi::Object)]
pub struct Client {
    inner: Mutex<libellule::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(
        instance: &Instance,
        username: &str,
        password: &str,
    ) -> Result<Client, LibelluleError> {
        let client = libellule::Client::login(&instance.inner, username, password).await?;

        Ok(Client {
            inner: Mutex::new(client),
        })
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub async fn get_periods(&self) -> Vec<Period> {
        self.inner
            .lock()
            .await
            .get_periods()
            .into_iter()
            .map(Period::from)
            .collect()
    }

    pub async fn get_default_period(&self) -> String {
        self.inner.lock().await.get_default_period()
    }

    pub async fn get_grades(&self, period: &Period) -> Result<GradesData, LibelluleError> {
        let period = models::Period {
            id: period.id.clone(),
            name: period.name.clone(),
        };

        let grades = self.inner.lock().await.get_grades(&period).await?;

        Ok(grades.into())
    }

    pub async fn timetable(&self, date: String) -> Result<Timetable, LibelluleError> {
        let date = Date::parse(&date, &Iso8601::DATE).map_err(libellule::error::Error::from)?;
        let timetable = self.inner.lock().await.timetable(date).await?;

        Ok(timetable.into())
    }
}
