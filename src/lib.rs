//! Serde support for Duration and chrono::Duration
//!
//! # Installation
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! serde_duration_ext = "0.1.0"
//! ```
//!
//! Also you can enable the `chrono` feature to support chrono::Duration
//!
//! ```toml
//! [dependencies]
//! serde_duration_ext = { version = "0.1.0", features = ["chrono"] }
//! ```
//!
//! # Usage
//!

mod durationunit;
mod timetunit;

pub mod error;

use std::time::Duration;

pub use durationunit::*;
use serde::{Deserialize, Deserializer};
pub use timetunit::*;

// re-export chrono if the feature is enabled
#[cfg(feature = "chrono")]
pub use chrono;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let duration_unit = DurationUnit::deserialize(deserializer)?;
    Ok(duration_unit.into())
}

#[cfg(feature = "chrono")]
pub mod chrono {
    use chrono::Duration;
    use serde::{Deserializer, Deserialize};

    use crate::DateTimeUnit;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let datetime_unit = DateTimeUnit::deserialize(deserializer)?;
        Ok(datetime_unit.into())
    }
}
