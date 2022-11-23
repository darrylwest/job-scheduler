/// RunAt - a cron like structure to specify when this job should run.
///
/// follows cron attributes: see https://www.ibm.com/docs/en/db2oc?topic=task-unix-cron-format for definitions
use chrono::{Datelike, NaiveDateTime, Timelike};
use log::{info, warn};
use serde::{Deserialize, Serialize};
// use chrono::Weekday;

/// RunAt - a cron like structure to specify when this job should run.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RunAt {
    /// range of 0..59 inclusive
    pub minutes: Vec<u8>, // 0..59
    /// range of 0..23 inclusive
    pub hours: Vec<u8>, // 0..23
    /// range of 1..31 inclusive
    pub days_of_month: Vec<u8>, // 1..31
    /// range of 0..7 inclusive (sunday == 0 or 7)
    pub days_of_week: Vec<u8>, // 1..7
    /// range of 1..12
    pub months: Vec<u8>, // 1..12
    /// range of current to 2999
    pub years: Vec<u16>, // current..2999
}

impl RunAt {
    /// create a new RunAt struct with zero length vec values
    pub fn new() -> RunAt {
        RunAt {
            minutes: Vec::new(),
            hours: Vec::new(),
            days_of_month: Vec::new(),
            days_of_week: Vec::new(),
            months: Vec::new(),
            years: Vec::new(),
        }
    }

    /// create a new RunAt struct with the supplied minutes; minutes are validated for 0..59 inclusive.  
    /// values outside that range are ignored
    pub fn with_minutes(minutes: &Vec<u8>) -> RunAt {
        let mut at = RunAt::new();

        for minute in minutes {
            if at.valid_minute(*minute) {
                at.minutes.push(*minute);
            } else {
                warn!("range error: {}, ignored", minute);
            }
        }

        info!("run at: {:?}", at);

        at
    }

    /// returns true if the RunAt struct is valid.  checks all vec values for the proper range.
    pub fn is_valid(&self) -> bool {
        true
    }

    /// return true if minute value is in the range of 0..59 inclusive, else false.
    pub fn valid_minute(&self, minute: u8) -> bool {
        self.validate_range(minute, 0u8, 59u8)
    }

    /// return true if hour value in the range of 0..23 inclusive, else false.
    pub fn valid_hour(&self, hour: u8) -> bool {
        self.validate_range(hour, 0u8, 23u8)
    }

    fn validate_range(&self, value: u8, min: u8, max: u8) -> bool {
        value >= min && value <= max
    }

    /// return true if the supplied date is
    pub fn match_datetime(&self, dt: &NaiveDateTime) -> bool {
        // return true if the vector is empty or the value is in the list
        fn match_list(value: u8, list: &Vec<u8>) -> bool {
            list.is_empty() || list.contains(&value)
        }

        fn match_years(value: u16, list: &Vec<u16>) -> bool {
            list.is_empty() || list.contains(&value)
        }

        let minute = dt.time().minute() as u8;
        let hour = dt.time().hour() as u8;
        let day_of_week = (dt.date().weekday().num_days_from_sunday() % 7) as u8;
        let day_of_month = dt.date().day() as u8;
        let month = dt.month() as u8;
        let year = dt.year() as u16;

        if !match_years(year, &self.years) {
            return false;
        }

        if !match_list(month, &self.months) {
            return false;
        }

        if !match_list(day_of_month, &self.days_of_month) {
            return false;
        }

        if !match_list(day_of_week, &self.days_of_week) {
            return false;
        }

        if match_list(hour, &self.hours) && match_list(minute, &self.minutes) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::naive::NaiveDateTime;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    fn parse_datetime(value: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(value, FORMAT).unwrap()
    }

    #[test]
    fn match_datetime_minutes() {
        // this should always return a match for 10, 20, 30, 40, 50 minutes past the hour
        let runat = RunAt::with_minutes(&vec![10u8, 20u8, 30u8, 40u8, 50u8]);

        let dt = parse_datetime("2022-01-01 10:00:00");
        assert_eq!(runat.match_datetime(&dt), false);

        let dt = parse_datetime("2022-01-01 20:10:00");
        assert_eq!(runat.match_datetime(&dt), true);

        let dt = parse_datetime("2022-01-01 00:20:00");
        assert_eq!(runat.match_datetime(&dt), true);

        let dt = parse_datetime("2022-01-01 01:30:00");
        assert_eq!(runat.match_datetime(&dt), true);
    }

    #[test]
    fn match_datetime_hours_and_minutes() {
        // this should always return a match for 10, 20, 30, 40, 50 minutes past the hour
        let mut runat = RunAt::with_minutes(&vec![10u8, 20u8, 30u8]);
        runat.hours = vec![10u8, 20u8, 0u8, 1u8];

        let dt = parse_datetime("2022-01-01 20:00:00");
        assert_eq!(runat.match_datetime(&dt), false);

        let dt = parse_datetime("2022-01-01 10:10:00");
        assert_eq!(runat.match_datetime(&dt), true);

        let dt = parse_datetime("2022-01-01 00:20:00");
        assert_eq!(runat.match_datetime(&dt), true);

        let dt = parse_datetime("2022-01-01 01:30:00");
        assert_eq!(runat.match_datetime(&dt), true);

        runat.years = vec![1950u16];
        let dt = parse_datetime("2022-01-01 01:30:00");
        assert_eq!(runat.match_datetime(&dt), false);

        runat.years = vec![2022u16];
        runat.months = vec![1u8, 3u8, 12u8];
        let dt = parse_datetime("2022-03-01 01:30:00");
        assert_eq!(runat.match_datetime(&dt), true);
    }

    #[test]
    fn new() {
        let runat = RunAt::new();

        assert!(runat.minutes.is_empty());
        assert!(runat.hours.is_empty());
        assert!(runat.days_of_month.is_empty());
        assert!(runat.days_of_week.is_empty());
        assert!(runat.months.is_empty());
        assert!(runat.years.is_empty());

        assert!(runat.is_valid())
    }

    #[test]
    fn with_minutes() {
        let minutes = vec![20u8, 30u8, 40u8];
        let runat = RunAt::with_minutes(&minutes);

        assert_eq!(runat.minutes, minutes);

        assert!(runat.hours.is_empty());
        assert!(runat.days_of_month.is_empty());
        assert!(runat.days_of_week.is_empty());
        assert!(runat.months.is_empty());
        assert!(runat.years.is_empty());

        assert!(runat.is_valid())
    }

    #[test]
    fn with_minutes_ignore() {
        let minutes = vec![20u8, 100u8];
        let runat = RunAt::with_minutes(&minutes);

        assert_ne!(runat.minutes, minutes);
        assert_eq!(runat.minutes, vec![20u8]);

        assert!(runat.hours.is_empty());
        assert!(runat.days_of_month.is_empty());
        assert!(runat.days_of_week.is_empty());
        assert!(runat.months.is_empty());
        assert!(runat.years.is_empty());

        assert!(runat.is_valid())
    }
}
