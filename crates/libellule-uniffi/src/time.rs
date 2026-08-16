use std::num::NonZero;

use time::format_description::well_known::Iso8601;
use time::format_description::well_known::iso8601::{
    Config, EncodedConfig, FormattedComponents, TimePrecision,
};
use uniffi::custom_type;
use uniffi::deps::anyhow;

type UniDate = time::Date;
type UniTime = time::Time;

const DATE_CONFIG: EncodedConfig = Config::DEFAULT
    .set_formatted_components(FormattedComponents::Date)
    .encode();

const DATE_FORMAT: &Iso8601<DATE_CONFIG> = &Iso8601;

const TIME_CONFIG: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: NonZero::new(0),
    })
    .set_formatted_components(FormattedComponents::Time)
    .encode();

const TIME_FORMAT: &Iso8601<TIME_CONFIG> = &Iso8601;

custom_type!(UniDate, String, {
    remote,
    lower: |t| t.format(DATE_FORMAT).unwrap(),
    try_lift: |s| UniDate::parse(&s, DATE_FORMAT).map_err(anyhow::Error::from)
});

custom_type!(UniTime, String, {
    remote,
    lower: |t| t.format(TIME_FORMAT).unwrap(),
    try_lift: |s| UniTime::parse(&s, TIME_FORMAT).map_err(anyhow::Error::from)
});
