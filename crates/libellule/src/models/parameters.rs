use std::collections::HashMap;

use serde::Serialize;

use crate::error::Error;
use crate::models::Tab;
use crate::protocol;

#[derive(Debug)]
pub struct Parameters {
    pub instance: Instance,
    pub user: User,
    pub tabs: TabsParameters,
}

#[derive(Debug)]
pub struct Instance {
    pub version: String,
    pub label: String,
    pub first_monday: String,
    pub first_day: String,
    pub last_day: String,
    pub places_per_day: u32,
    pub places_per_hour: u32,
    pub start_hours: Vec<String>,
    pub end_hours: Vec<String>,
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

        let start_hours = general
            .start_hours
            .value
            .into_iter()
            .map(|start_hour| start_hour.label)
            .collect();

        let end_hours = general
            .end_hours
            .value
            .into_iter()
            .map(|end_hour| end_hour.label)
            .collect();

        let instance = Instance {
            version: general.version,
            label: general.label,
            first_monday: general.first_monday.value,
            first_day: general.first_day.value,
            last_day: general.last_day.value,
            places_per_day: general.places_per_day,
            places_per_hour: general.places_per_hour,
            start_hours,
            end_hours,
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
