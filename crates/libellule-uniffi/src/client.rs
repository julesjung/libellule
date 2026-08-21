use time::Date;

use libellule::error::Error;
use libellule::model::{BoundaryDates, GradesData, Homework, Menu, Period, Timetable};

use crate::instance::Instance;

#[derive(uniffi::Object)]
pub struct Client {
    inner: libellule::Client,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(instance: &Instance, username: &str, password: &str) -> Result<Client, Error> {
        let client = libellule::Client::login(&instance.inner, username, password).await?;

        Ok(Client { inner: client })
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub fn periods(&self) -> Vec<Period> {
        self.inner.periods()
    }

    pub fn default_period(&self) -> String {
        self.inner.default_period()
    }

    pub async fn get_grades(&self, period: &Period) -> Result<GradesData, Error> {
        let grades = self.inner.grades(period).await?;

        Ok(grades)
    }

    pub fn boundary_dates(&self) -> BoundaryDates {
        self.inner.boundary_dates()
    }

    pub async fn timetable(&self, date: Date) -> Result<Timetable, Error> {
        let timetable = self.inner.timetable(date).await?;

        Ok(timetable)
    }

    pub async fn menu(&self, date: Date) -> Result<Menu, Error> {
        let menu = self.inner.menu(date).await?;

        Ok(menu)
    }

    pub async fn homework(&self, date: Date) -> Result<Homework, Error> {
        let homework = self.inner.homework(date).await?;

        Ok(homework)
    }
}
