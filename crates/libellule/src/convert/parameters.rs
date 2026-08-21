use time::Duration;

use crate::convert::value::date;
use crate::error::ConversionError;
use crate::model;
use crate::protocol;
use crate::time::parse_time;

pub(crate) fn parameters(
    raw_instance: protocol::InstanceParameters,
    raw_user: protocol::UserParameters,
) -> Result<model::Parameters, ConversionError> {
    let general = raw_instance.general;

    let place_duration = Duration::hours(1) / general.places_per_hour;
    let start_time = general
        .start_hours
        .0
        .into_iter()
        .find_map(|start_hour| match start_hour.id == 0 {
            true => Some(start_hour.label),
            false => None,
        })
        .ok_or(ConversionError::Parse)?;

    let start_time = parse_time(&start_time).map_err(|_| ConversionError::Parse)?;

    let end_time = start_time + place_duration * general.places_per_day;

    let instance = model::Instance {
        version: general.version,
        label: general.label,
        first_monday: date(general.first_monday)?,
        first_day: date(general.first_day).map_err(|_| ConversionError::Parse)?,
        last_day: date(general.last_day).map_err(|_| ConversionError::Parse)?,
        places_per_day: general.places_per_day,
        places_per_hour: general.places_per_hour,
        place_duration,
        start_time,
        end_time,
        periods: general
            .periods
            .into_iter()
            .map(model::Period::from)
            .collect(),
    };

    let user = model::User {
        id: raw_user.resources.id,
        fullname: raw_user.resources.label,
        kind: raw_user.resources.group,
    };
    let tabs_periods = raw_user
        .resources
        .tabs_periods
        .0
        .into_iter()
        .filter_map(|tab_periods| tab_periods.try_into().ok())
        .collect();

    let tabs = model::TabsParameters {
        periods: tabs_periods,
    };

    Ok(model::Parameters {
        instance,
        user,
        tabs,
    })
}

impl From<protocol::Period> for model::Period {
    fn from(value: protocol::Period) -> Self {
        model::Period {
            id: value.id.unwrap(),
            name: value.name,
        }
    }
}

impl From<model::Period> for protocol::Period {
    fn from(value: model::Period) -> Self {
        protocol::Period {
            id: Some(value.id),
            name: value.name,
        }
    }
}

impl TryFrom<protocol::TabPeriods> for (model::Tab, model::TabPeriods) {
    type Error = ConversionError;

    fn try_from(value: protocol::TabPeriods) -> Result<Self, ConversionError> {
        value.id.try_into().map(|tab: model::Tab| {
            (
                tab,
                model::TabPeriods {
                    periods: value
                        .periods
                        .0
                        .into_iter()
                        .map(model::Period::from)
                        .collect(),
                    default: value.default.0.id.unwrap(),
                },
            )
        })
    }
}
