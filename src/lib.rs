pub mod constants;
pub mod errors;

use std::str::FromStr;
use std::fmt;


/// The unit of time
#[derive(Debug, PartialEq)]
pub enum TimeUnit {
    /// Nanosecond = 1 Nanosecond
    Nanoseconds,

    /// 1 Microsecond = 1000 Nanoseconds
    Microseconds,

    /// 1 Milliseconds = 1e+6 Nanoseconds
    Milliseconds,

    /// 1 Second = 1e+9 Nanoseconds
    Seconds,

    /// 1 Minute = 6e+10 Nanoseconds
    Minutes,

    /// 1 Hour = 3.6e+12 Nanoseconds
    Hours,

    /// 1 Day = 8.64e+13 Nanoseconds
    Days,

    /// 1 Week = 6.048e+14 Nanoseconds
    Weeks,

    /// 1 Month = 2.628e+15 Nanoseconds
    Months,

    /// 1 Year = 3.154e+16 Nanosecondss
    Years,
}


impl TimeUnit {
    /// Get nanoseconds in `TimeUnit`.
    pub fn get_unit_nanoseconds(self) -> u64 {
        match self {
            TimeUnit::Nanoseconds => constants::u64::NANOSECOND,
            TimeUnit::Microseconds => constants::u64::MICROSECOND,
            TimeUnit::Milliseconds => constants::u64::MILLISECOND,
            TimeUnit::Seconds => constants::u64::SECOND,
            TimeUnit::Minutes => constants::u64::MINUTE,
            TimeUnit::Hours => constants::u64::HOUR,
            TimeUnit::Days => constants::u64::DAY,
            TimeUnit::Weeks => constants::u64::WEEK,
            TimeUnit::Months => constants::u64::MONTH,
            TimeUnit::Years => constants::u64::YEAR,
        }
    }
}

impl FromStr for TimeUnit {
    type Err = errors::UnrecognizedUnitError;

    /// Get an instance of `TimeUnit` from a string slice.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit_str = s.trim();
        match unit_str {
            ""  => Ok(TimeUnit::Nanoseconds),
            "Nanoseconds"  => Ok(TimeUnit::Nanoseconds),
            "Microseconds"  => Ok(TimeUnit::Microseconds),
            "Milliseconds"  => Ok(TimeUnit::Milliseconds),
            "Seconds"  => Ok(TimeUnit::Seconds),
            "Minutes"  => Ok(TimeUnit::Minutes),
            "Hours"  => Ok(TimeUnit::Hours),
            "Days"  => Ok(TimeUnit::Days),
            "Weeks"  => Ok(TimeUnit::Weeks),
            "Months"  => Ok(TimeUnit::Months),
            "Years"  => Ok(TimeUnit::Years),
            _ => Err(errors::UnrecognizedUnitError {}),
        }
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(TimeUnit::Nanoseconds, TimeUnit::from_str("").unwrap());
        assert_eq!(TimeUnit::Nanoseconds, TimeUnit::from_str("Nanoseconds").unwrap());
        assert_eq!(TimeUnit::Microseconds, TimeUnit::from_str("Microseconds").unwrap());
        assert_eq!(TimeUnit::Milliseconds, TimeUnit::from_str("Milliseconds").unwrap());
        assert_eq!(TimeUnit::Seconds, TimeUnit::from_str("Seconds").unwrap());
        assert_eq!(TimeUnit::Minutes, TimeUnit::from_str("Minutes").unwrap());
        assert_eq!(TimeUnit::Hours, TimeUnit::from_str("Hours").unwrap());
        assert_eq!(TimeUnit::Days, TimeUnit::from_str("Days").unwrap());
        assert_eq!(TimeUnit::Weeks, TimeUnit::from_str("Weeks").unwrap());
        assert_eq!(TimeUnit::Months, TimeUnit::from_str("Months").unwrap());
        assert_eq!(TimeUnit::Years, TimeUnit::from_str("Years").unwrap());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(String::from("Nanoseconds"), TimeUnit::Nanoseconds.to_string());
        assert_eq!(String::from("Microseconds"), TimeUnit::Microseconds.to_string());
        assert_eq!(String::from("Milliseconds"), TimeUnit::Milliseconds.to_string());
        assert_eq!(String::from("Seconds"), TimeUnit::Seconds.to_string());
        assert_eq!(String::from("Minutes"), TimeUnit::Minutes.to_string());
        assert_eq!(String::from("Hours"), TimeUnit::Hours.to_string());
        assert_eq!(String::from("Days"), TimeUnit::Days.to_string());
        assert_eq!(String::from("Weeks"), TimeUnit::Weeks.to_string());
        assert_eq!(String::from("Months"), TimeUnit::Months.to_string());
        assert_eq!(String::from("Years"), TimeUnit::Years.to_string());
    }

    #[test]
    fn test_get_unit_nanoseconds() {
        assert_eq!(TimeUnit::Nanoseconds.get_unit_nanoseconds(), 1);
        assert_eq!(TimeUnit::Microseconds.get_unit_nanoseconds(), 1_000);
        assert_eq!(TimeUnit::Milliseconds.get_unit_nanoseconds(), 1_000_000);
        assert_eq!(TimeUnit::Seconds.get_unit_nanoseconds(), 1_000_000_000);
        assert_eq!(TimeUnit::Minutes.get_unit_nanoseconds(), 60_000_000_000);
        assert_eq!(TimeUnit::Hours.get_unit_nanoseconds(), 3_600_000_000_000);
        assert_eq!(TimeUnit::Days.get_unit_nanoseconds(), 86_400_000_000_000);
        assert_eq!(TimeUnit::Weeks.get_unit_nanoseconds(), 604_800_000_000_000);
        assert_eq!(TimeUnit::Months.get_unit_nanoseconds(), 2_628_000_000_000_000);
        assert_eq!(TimeUnit::Years.get_unit_nanoseconds(), 31_540_000_000_000_000);
    }
}
