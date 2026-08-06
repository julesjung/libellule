use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::Tab;
use crate::protocol;

#[derive(Debug, Deserialize)]
#[serde(try_from = "protocol::UserParameters")]
pub struct UserParameters {
    pub user: User,
    pub tabs: TabsParameters,
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

impl TryFrom<protocol::UserParameters> for UserParameters {
    type Error = Error;

    fn try_from(value: protocol::UserParameters) -> Result<Self, Error> {
        let tabs_periods: HashMap<Tab, TabPeriods> = value
            .resources
            .tabs_periods
            .value
            .into_iter()
            .filter_map(|tab_periods| tab_periods.try_into().ok())
            .collect();

        let user = User {
            id: value.resources.id,
            fullname: value.resources.label,
            kind: value.resources.group,
        };

        Ok(UserParameters {
            user,
            tabs: TabsParameters {
                periods: tabs_periods,
            },
        })
    }
}
