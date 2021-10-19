pub mod constants;

/// The unit of time
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


#[cfg(test)]
mod tests {
    use crate::TimeUnit;

    #[test]
    fn get_unit_nanoseconds_works_for_nanoseconds() {
        assert_eq!(TimeUnit::Nanoseconds.get_unit_nanoseconds(), 1);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_microseconds() {
        assert_eq!(TimeUnit::Microseconds.get_unit_nanoseconds(), 1_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_milliseconds() {
        assert_eq!(TimeUnit::Milliseconds.get_unit_nanoseconds(), 1_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_seconds() {
        assert_eq!(TimeUnit::Seconds.get_unit_nanoseconds(), 1_000_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_minutes() {
        assert_eq!(TimeUnit::Minutes.get_unit_nanoseconds(), 60_000_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_hours() {
        assert_eq!(TimeUnit::Hours.get_unit_nanoseconds(), 3_600_000_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_days() {
        assert_eq!(TimeUnit::Days.get_unit_nanoseconds(), 86_400_000_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_weeks() {
        assert_eq!(TimeUnit::Weeks.get_unit_nanoseconds(), 604_800_000_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_months() {
        assert_eq!(TimeUnit::Months.get_unit_nanoseconds(), 2_628_000_000_000_000);
    }

    #[test]
    fn get_unit_nanoseconds_works_for_years() {
        assert_eq!(TimeUnit::Years.get_unit_nanoseconds(), 31_540_000_000_000_000);
    }
}
