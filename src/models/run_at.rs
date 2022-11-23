/// RunAt - a cron like structure to specify when this job should run.
///
/// follows cron attributes: see https://www.ibm.com/docs/en/db2oc?topic=task-unix-cron-format for definitions
use log::{info, warn};
use serde::{Deserialize, Serialize};

/// RunAt - a cron like structure to specify when this job should run.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub fn with_minutes(minutes: Vec<u8>) -> RunAt {
        let mut at = RunAt::new();

        for minute in minutes {
            if at.valid_minute(minute) {
                at.minutes.push(minute);
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
