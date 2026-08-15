use libellule::model;
use time::Date;
use time::format_description::well_known::Iso8601;

use crate::error::LibelluleError;
use crate::grades::{GradesData, Period};
use crate::instance::Instance;
use crate::menu::Menu;
use crate::timetable::{BoundaryDates, Timetable};

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
        self.inner
            .get_periods()
            .into_iter()
            .map(Period::from)
            .collect()
    }

    pub fn get_default_period(&self) -> String {
        self.inner.get_default_period()
    }

    pub async fn get_grades(&self, period: &Period) -> Result<GradesData, LibelluleError> {
        let period = model::Period {
            id: period.id.clone(),
            name: period.name.clone(),
        };

        let grades = self.inner.get_grades(&period).await?;

        Ok(grades.into())
    }

    pub fn boundary_dates(&self) -> BoundaryDates {
        self.inner.boundary_dates().into()
    }

    pub async fn timetable(&self, date: String) -> Result<Timetable, LibelluleError> {
        let date = Date::parse(&date, &Iso8601::DATE).map_err(libellule::error::Error::from)?;
        let timetable = self.inner.timetable(date).await?;

        Ok(timetable.into())
    }

    pub async fn menu(&self, date: String) -> Result<Menu, LibelluleError> {
        let date = Date::parse(&date, &Iso8601::DATE).map_err(libellule::error::Error::from)?;
        let menu = self.inner.menu(date).await?;

        Ok(menu.into())
    }
}
