use std::collections::HashMap;

use serde::Serialize;
use time::{Date, Duration, Time};

use crate::error::Error;
use crate::model::Tab;
use crate::protocol;
use crate::time::{parse_date, parse_time};

#[derive(Debug)]
pub struct Parameters {
    pub instance: Instance,
    pub user: User,
    pub tabs: TabsParameters,
}

impl Parameters {
    pub fn place_to_time(&self, place: u32) -> Time {
        self.instance.start_time + self.instance.place_duration * place
    }
}

#[derive(Debug)]
pub struct Instance {
    pub version: String,
    pub label: String,
    pub first_monday: Date,
    pub first_day: Date,
    pub last_day: Date,
    pub places_per_day: u32,
    pub places_per_hour: u32,
    pub place_duration: Duration,
    pub start_time: Time,
    pub end_time: Time,
    pub periods: Vec<Period>,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub fullname: String,
    pub kind: u32,
}

#[derive(Debug)]
pub struct TabsParameters {
    pub periods: HashMap<Tab, TabPeriods>,
}

#[derive(Debug, Clone)]
pub struct TabPeriods {
    pub periods: Vec<Period>,
    pub default: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "protocol::Period")]
pub struct Period {
    pub id: String,
    pub name: String,
}

impl From<protocol::Period> for Period {
    fn from(value: protocol::Period) -> Self {
        Period {
            id: value.id.unwrap(),
            name: value.name,
        }
    }
}

impl From<Period> for protocol::Period {
    fn from(value: Period) -> Self {
        protocol::Period {
            id: Some(value.id),
            name: value.name,
        }
    }
}

impl TryFrom<protocol::TabPeriods> for (Tab, TabPeriods) {
    type Error = Error;

    fn try_from(value: protocol::TabPeriods) -> Result<Self, Error> {
        value.id.try_into().map(|tab: Tab| {
            (
                tab,
                TabPeriods {
                    periods: value
                        .periods
                        .value
                        .into_iter()
                        .map(|period| period.into())
                        .collect(),
                    default: value.default.value.id.unwrap(),
                },
            )
        })
    }
}

impl TryFrom<(protocol::InstanceParameters, protocol::UserParameters)> for Parameters {
    type Error = Error;

    fn try_from(
        value: (protocol::InstanceParameters, protocol::UserParameters),
    ) -> Result<Self, Error> {
        let (instance_parameters, user_parameters) = value;

        let tabs_periods: HashMap<Tab, TabPeriods> = user_parameters
            .resources
            .tabs_periods
            .value
            .into_iter()
            .filter_map(|tab_periods| tab_periods.try_into().ok())
            .collect();

        let general = instance_parameters.general;

        let place_duration = Duration::hours(1) / general.places_per_hour;
        let start_time = general
            .start_hours
            .value
            .into_iter()
            .find_map(|start_hour| match start_hour.id == 0 {
                true => Some(start_hour.label),
                false => None,
            })
            .ok_or(Error::StartHourNotFound)?;

        let start_time = parse_time(&start_time)?;

        let end_time = start_time + place_duration * general.places_per_day;

        let instance = Instance {
            version: general.version,
            label: general.label,
            first_monday: parse_date(general.first_monday.value.as_str())?,
            first_day: parse_date(general.first_day.value.as_str())?,
            last_day: parse_date(general.last_day.value.as_str())?,
            places_per_day: general.places_per_day,
            places_per_hour: general.places_per_hour,
            place_duration,
            start_time,
            end_time,
            periods: general.periods.into_iter().map(Period::from).collect(),
        };

        let user = User {
            id: user_parameters.resources.id,
            fullname: user_parameters.resources.label,
            kind: user_parameters.resources.group,
        };

        let tabs = TabsParameters {
            periods: tabs_periods,
        };

        Ok(Parameters {
            instance,
            user,
            tabs,
        })
    }
}
