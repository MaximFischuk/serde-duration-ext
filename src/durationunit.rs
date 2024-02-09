use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::error::Error;
use crate::TimeUnit;

lazy_static! {
    static ref DURATION_REGEX: Regex =
        Regex::new(r"^(?P<value>\d+)(?P<unit>ns|us|ms|s|m|h|d|w){1}$")
            .expect("Regex compilation error");
}

/// The number of seconds in a minute.
const SECS_PER_MINUTE: u64 = 60;
/// The number of seconds in an hour.
const SECS_PER_HOUR: u64 = 3600;
/// The number of (non-leap) seconds in days.
const SECS_PER_DAY: u64 = 86_400;
/// The number of (non-leap) seconds in a week.
const SECS_PER_WEEK: u64 = 604_800;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct DurationUnit {
    value: u64,
    unit: TimeUnit,
}

impl FromStr for DurationUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if DURATION_REGEX.is_match(s) {
            let caps = DURATION_REGEX
                .captures(s)
                .ok_or_else(|| Error::StringDoesNotMatchRegex)?;
            let value = caps.name("value").unwrap().as_str().parse().unwrap();
            let time_unit = caps.name("unit").unwrap().as_str();
            let unit = time_unit.parse::<TimeUnit>()?;
            Ok(Self { value, unit })
        } else {
            Err(Error::Syntax(
                "Current string is not correct duration unit value".to_owned(),
            ))
        }
    }
}

impl DurationUnit {
    /// Creates a new `DurationUnit` from the specified value and time unit.
    pub fn new(value: u64, unit: TimeUnit) -> Self {
        Self { value, unit }
    }

    /// Returns the time unit of this duration unit.
    pub fn unit(&self) -> TimeUnit {
        self.unit
    }

    pub fn as_secs(&self) -> u64 {
        match self.unit {
            TimeUnit::Nanosecond => self.value / 1_000_000_000,
            TimeUnit::Microsecond => self.value / 1_000_000,
            TimeUnit::Millisecond => self.value / 1_000,
            TimeUnit::Second => self.value,
            TimeUnit::Minute => self.value * SECS_PER_MINUTE,
            TimeUnit::Hour => self.value * SECS_PER_HOUR,
            TimeUnit::Day => self.value * SECS_PER_DAY,
            TimeUnit::Week => self.value * SECS_PER_WEEK,
        }
    }
}

impl From<DurationUnit> for std::time::Duration {
    fn from(duration_unit: DurationUnit) -> Self {
        match duration_unit.unit {
            TimeUnit::Nanosecond => std::time::Duration::from_nanos(duration_unit.value),
            TimeUnit::Microsecond => std::time::Duration::from_micros(duration_unit.value),
            TimeUnit::Millisecond => std::time::Duration::from_millis(duration_unit.value),
            TimeUnit::Second => std::time::Duration::from_secs(duration_unit.value),
            TimeUnit::Minute => {
                std::time::Duration::from_secs(duration_unit.value * SECS_PER_MINUTE)
            }
            TimeUnit::Hour => std::time::Duration::from_secs(duration_unit.value * SECS_PER_HOUR),
            TimeUnit::Day => std::time::Duration::from_secs(duration_unit.value * SECS_PER_DAY),
            TimeUnit::Week => std::time::Duration::from_secs(duration_unit.value * SECS_PER_WEEK),
        }
    }
}

#[cfg(feature = "chrono")]
impl From<DurationUnit> for chrono::Duration {
    fn from(duration_unit: DurationUnit) -> Self {
        match duration_unit.unit {
            TimeUnit::Nanosecond => chrono::Duration::nanoseconds(duration_unit.value as i64),
            TimeUnit::Microsecond => chrono::Duration::microseconds(duration_unit.value as i64),
            TimeUnit::Millisecond => chrono::Duration::milliseconds(duration_unit.value as i64),
            TimeUnit::Second => chrono::Duration::seconds(duration_unit.value as i64),
            TimeUnit::Minute => chrono::Duration::minutes(duration_unit.value as i64),
            TimeUnit::Hour => chrono::Duration::hours(duration_unit.value as i64),
            TimeUnit::Day => chrono::Duration::days(duration_unit.value as i64),
            TimeUnit::Week => chrono::Duration::weeks(duration_unit.value as i64),
        }
    }
}

impl<'a> Deserialize<'a> for DurationUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_duration_unit_from_str() {
        let duration_unit = "10s".parse::<DurationUnit>().unwrap();
        assert_eq!(duration_unit.value, 10);
        assert_eq!(duration_unit.unit, TimeUnit::Second);

        let duration_unit = "500ms".parse::<DurationUnit>().unwrap();
        assert_eq!(duration_unit.value, 500);
        assert_eq!(duration_unit.unit, TimeUnit::Millisecond);

        let duration_unit = "1h".parse::<DurationUnit>().unwrap();
        assert_eq!(duration_unit.value, 1);
        assert_eq!(duration_unit.unit, TimeUnit::Hour);

        let duration_unit = "100us".parse::<DurationUnit>().unwrap();
        assert_eq!(duration_unit.value, 100);
        assert_eq!(duration_unit.unit, TimeUnit::Microsecond);

        let duration_unit = "2d".parse::<DurationUnit>().unwrap();
        assert_eq!(duration_unit.value, 2);
        assert_eq!(duration_unit.unit, TimeUnit::Day);

        let duration_unit = "1w".parse::<DurationUnit>().unwrap();
        assert_eq!(duration_unit.value, 1);
        assert_eq!(duration_unit.unit, TimeUnit::Week);

        let duration_unit = "invalid".parse::<DurationUnit>();
        assert!(duration_unit.is_err());
    }

    #[test]
    fn test_duration_unit_into_duration() {
        let duration_unit = DurationUnit {
            value: 10,
            unit: TimeUnit::Second,
        };
        let duration: Duration = duration_unit.into();
        assert_eq!(duration, Duration::from_secs(10));

        let duration_unit = DurationUnit {
            value: 500,
            unit: TimeUnit::Millisecond,
        };
        let duration: Duration = duration_unit.into();
        assert_eq!(duration, Duration::from_millis(500));

        let duration_unit = DurationUnit {
            value: 1,
            unit: TimeUnit::Hour,
        };
        let duration: Duration = duration_unit.into();
        assert_eq!(duration, Duration::from_secs(3600));

        let duration_unit = DurationUnit {
            value: 100,
            unit: TimeUnit::Microsecond,
        };
        let duration: Duration = duration_unit.into();
        assert_eq!(duration, Duration::from_micros(100));

        let duration_unit = DurationUnit {
            value: 2,
            unit: TimeUnit::Day,
        };
        let duration: Duration = duration_unit.into();
        assert_eq!(duration, Duration::from_secs(172_800));

        let duration_unit = DurationUnit {
            value: 1,
            unit: TimeUnit::Week,
        };
        let duration: Duration = duration_unit.into();
        assert_eq!(duration, Duration::from_secs(604_800));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_duration_unit_into_chrono_duration() {
        let duration_unit = DurationUnit {
            value: 10,
            unit: TimeUnit::Second,
        };
        let duration: chrono::Duration = duration_unit.into();
        assert_eq!(duration, chrono::Duration::seconds(10));

        let duration_unit = DurationUnit {
            value: 500,
            unit: TimeUnit::Millisecond,
        };
        let duration: chrono::Duration = duration_unit.into();
        assert_eq!(duration, chrono::Duration::milliseconds(500));

        let duration_unit = DurationUnit {
            value: 1,
            unit: TimeUnit::Hour,
        };
        let duration: chrono::Duration = duration_unit.into();
        assert_eq!(duration, chrono::Duration::hours(1));

        let duration_unit = DurationUnit {
            value: 100,
            unit: TimeUnit::Microsecond,
        };
        let duration: chrono::Duration = duration_unit.into();
        assert_eq!(duration, chrono::Duration::microseconds(100));

        let duration_unit = DurationUnit {
            value: 2,
            unit: TimeUnit::Day,
        };
        let duration: chrono::Duration = duration_unit.into();
        assert_eq!(duration, chrono::Duration::days(2));

        let duration_unit = DurationUnit {
            value: 1,
            unit: TimeUnit::Week,
        };
        let duration: chrono::Duration = duration_unit.into();
        assert_eq!(duration, chrono::Duration::weeks(1));
    }

    #[test]
    fn test_deseialize_duration_unit() {
        let duration_unit = serde_json::from_str::<DurationUnit>("\"10s\"").unwrap();
        assert_eq!(duration_unit.value, 10);
        assert_eq!(duration_unit.unit, TimeUnit::Second);

        let duration_unit = serde_json::from_str::<DurationUnit>("\"500ms\"").unwrap();
        assert_eq!(duration_unit.value, 500);
        assert_eq!(duration_unit.unit, TimeUnit::Millisecond);

        let duration_unit = serde_json::from_str::<DurationUnit>("\"1h\"").unwrap();
        assert_eq!(duration_unit.value, 1);
        assert_eq!(duration_unit.unit, TimeUnit::Hour);

        let duration_unit = serde_json::from_str::<DurationUnit>("\"100us\"").unwrap();
        assert_eq!(duration_unit.value, 100);
        assert_eq!(duration_unit.unit, TimeUnit::Microsecond);

        let duration_unit = serde_json::from_str::<DurationUnit>("\"2d\"").unwrap();
        assert_eq!(duration_unit.value, 2);
        assert_eq!(duration_unit.unit, TimeUnit::Day);

        let duration_unit = serde_json::from_str::<DurationUnit>("\"1w\"").unwrap();
        assert_eq!(duration_unit.value, 1);
        assert_eq!(duration_unit.unit, TimeUnit::Week);

        let duration_unit = serde_json::from_str::<DurationUnit>("\"invalid\"");
        assert!(duration_unit.is_err());
    }
}
