use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api;
use crate::error::Error;
use crate::models::Tab;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[serde(try_from = "api::UserParameters")]
pub struct UserParameters {
    pub fullname: String,
    pub class: String,
    pub tabs: TabsParameters,
}

#[derive(Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct TabsParameters {
    pub periods: HashMap<Tab, TabPeriods>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct TabPeriods {
    pub periods: Vec<Period>,
    pub default: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[serde(into = "api::Period")]
pub struct Period {
    pub id: String,
    pub name: String,
}

impl From<api::Period> for Period {
    fn from(value: api::Period) -> Self {
        Period {
            id: value.id.unwrap(),
            name: value.name,
        }
    }
}

impl From<Period> for api::Period {
    fn from(value: Period) -> Self {
        api::Period {
            id: Some(value.id),
            name: value.name,
        }
    }
}

impl TryFrom<api::TabPeriods> for (Tab, TabPeriods) {
    type Error = Error;

    fn try_from(value: api::TabPeriods) -> Result<Self, Error> {
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

impl TryFrom<api::UserParameters> for UserParameters {
    type Error = Error;

    fn try_from(value: api::UserParameters) -> Result<Self, Error> {
        let tabs_periods: HashMap<Tab, TabPeriods> = value
            .resources
            .tabs_periods
            .value
            .into_iter()
            .filter_map(|tab_periods| tab_periods.try_into().ok())
            .collect();

        Ok(UserParameters {
            fullname: value.resources.label,
            class: value.resources.class.name,
            tabs: TabsParameters {
                periods: tabs_periods,
            },
        })
    }
}
