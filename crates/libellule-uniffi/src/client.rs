use libellule::model::{BoundaryDates, Period, Timetable};
use libellule::model::{GradesData, Menu};
use time::Date;
use time::format_description::well_known::Iso8601;

use crate::error::LibelluleError;
use crate::instance::Instance;

#[derive(uniffi::Object)]
pub struct Client {
    inner: libellule::Client,
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

        Ok(Client { inner: client })
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub fn get_periods(&self) -> Vec<Period> {
        self.inner.get_periods()
    }

    pub fn get_default_period(&self) -> String {
        self.inner.get_default_period()
    }

    pub async fn get_grades(&self, period: &Period) -> Result<GradesData, LibelluleError> {
        let grades = self.inner.get_grades(period).await?;

        Ok(grades)
    }

    pub fn boundary_dates(&self) -> BoundaryDates {
        self.inner.boundary_dates()
    }

    pub async fn timetable(&self, date: Date) -> Result<Timetable, LibelluleError> {
        let timetable = self.inner.timetable(date).await?;

        Ok(timetable)
    }

    pub async fn menu(&self, date: String) -> Result<Menu, LibelluleError> {
        let date = Date::parse(&date, &Iso8601::DATE).map_err(libellule::error::Error::from)?;
        let menu = self.inner.menu(date).await?;

        Ok(menu)
    }
}
