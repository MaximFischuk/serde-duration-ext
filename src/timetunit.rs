use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum TimeUnit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
}

impl FromStr for TimeUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" | "nanosecond" | "nanos" | "nanoseconds" => Ok(TimeUnit::Nanosecond),
            "us" | "microsecond" | "micros" | "microseconds" => Ok(TimeUnit::Microsecond),
            "ms" | "millisecond" | "millis" | "milliseconds" => Ok(TimeUnit::Millisecond),
            "s" | "second" | "secs" | "seconds" => Ok(TimeUnit::Second),
            "m" | "minute" | "mins" | "minutes" => Ok(TimeUnit::Minute),
            "h" | "hour" | "hours" => Ok(TimeUnit::Hour),
            "d" | "day" | "days" => Ok(TimeUnit::Day),
            "w" | "week" | "weeks" => Ok(TimeUnit::Week),
            _ => Err(Error::UnitNotSupported(format!(
                "Unit '{}' not supported",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("ns".parse(), Ok(TimeUnit::Nanosecond));
        assert_eq!("nanosecond".parse(), Ok(TimeUnit::Nanosecond));
        assert_eq!("nanos".parse(), Ok(TimeUnit::Nanosecond));
        assert_eq!("nanoseconds".parse(), Ok(TimeUnit::Nanosecond));

        assert_eq!("us".parse(), Ok(TimeUnit::Microsecond));
        assert_eq!("microsecond".parse(), Ok(TimeUnit::Microsecond));
        assert_eq!("micros".parse(), Ok(TimeUnit::Microsecond));
        assert_eq!("microseconds".parse(), Ok(TimeUnit::Microsecond));

        assert_eq!("ms".parse(), Ok(TimeUnit::Millisecond));
        assert_eq!("millisecond".parse(), Ok(TimeUnit::Millisecond));
        assert_eq!("millis".parse(), Ok(TimeUnit::Millisecond));
        assert_eq!("milliseconds".parse(), Ok(TimeUnit::Millisecond));

        assert_eq!("s".parse(), Ok(TimeUnit::Second));
        assert_eq!("second".parse(), Ok(TimeUnit::Second));
        assert_eq!("secs".parse(), Ok(TimeUnit::Second));
        assert_eq!("seconds".parse(), Ok(TimeUnit::Second));

        assert_eq!("m".parse(), Ok(TimeUnit::Minute));
        assert_eq!("minute".parse(), Ok(TimeUnit::Minute));
        assert_eq!("mins".parse(), Ok(TimeUnit::Minute));
        assert_eq!("minutes".parse(), Ok(TimeUnit::Minute));

        assert_eq!("h".parse(), Ok(TimeUnit::Hour));
        assert_eq!("hour".parse(), Ok(TimeUnit::Hour));
        assert_eq!("hours".parse(), Ok(TimeUnit::Hour));

        assert_eq!("d".parse(), Ok(TimeUnit::Day));
        assert_eq!("day".parse(), Ok(TimeUnit::Day));
        assert_eq!("days".parse(), Ok(TimeUnit::Day));

        assert_eq!("w".parse(), Ok(TimeUnit::Week));
        assert_eq!("week".parse(), Ok(TimeUnit::Week));
        assert_eq!("weeks".parse(), Ok(TimeUnit::Week));

        assert!("foo".parse::<TimeUnit>().is_err());
    }

    #[test]
    fn test_ord() {
        assert!(TimeUnit::Nanosecond < TimeUnit::Microsecond);
        assert!(TimeUnit::Microsecond < TimeUnit::Millisecond);
        assert!(TimeUnit::Millisecond < TimeUnit::Second);
        assert!(TimeUnit::Second < TimeUnit::Minute);
        assert!(TimeUnit::Minute < TimeUnit::Hour);
        assert!(TimeUnit::Hour < TimeUnit::Day);
        assert!(TimeUnit::Day < TimeUnit::Week);
    }
}
