use chrono::{Duration, NaiveDateTime, Utc};
use std::ops::Add;

/// Format string for date with a T (like rfc-3339 without the timezone info
/// since the timezone is always Taiwan's)
/// ex: 2021-06-27T01:23:45
const FORMAT_WITH_T: &str = "%Y-%m-%dT%H:%M:%S";

/// Format string for parsing the JSON from the api.
/// ex: 2021-06-27 01:23:45
const FORMAT_WITHOUT_T: &str = "%Y-%m-%d %H:%M:%S";

/// get the local date for new sqlite databases
/// always use Taiwan local time, so add 8 hours to UTC
/// couldn't figure out how to change the timezone the smart way :(
pub fn get_local_date_time_with_t() -> String {
    Utc::now()
        .add(Duration::hours(8))
        .format(FORMAT_WITH_T)
        .to_string()
}

/// a struct to make doing time things easier
pub struct CwbTime(NaiveDateTime);

impl CwbTime {
    /// take a time, and format it with T (for api)
    pub fn format_with_t(&self) -> String {
        self.0.format(FORMAT_WITH_T).to_string()
    }

    /// parse a time string with a T
    pub fn new_time_string_with_t(date_time_string: &str) -> Self {
        Self(NaiveDateTime::parse_from_str(date_time_string, FORMAT_WITH_T).unwrap())
    }

    /// parse a time string without a T
    pub fn new_time_string_without_t(date_time_string: &str) -> Self {
        Self(NaiveDateTime::parse_from_str(date_time_string, FORMAT_WITHOUT_T).unwrap())
    }

    /// add some seconds (this helps me not get duplicate earthquakes)
    pub fn add_seconds(&mut self, seconds: i64) -> &mut Self {
        self.0 = self.0.add(Duration::seconds(seconds));
        self
    }

    /// return the NaiveDateTime value from self.0
    pub fn get_date_time(&self) -> NaiveDateTime {
        self.0
    }
}
