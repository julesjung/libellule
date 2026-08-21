use std::collections::HashMap;

use serde::Serialize;
use time::{Date, Duration, Time};

use crate::model::Tab;
use crate::protocol;

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
