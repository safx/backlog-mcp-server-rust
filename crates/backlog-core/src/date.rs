use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{self, Deserializer, Serializer};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

const SER_FORMAT: &str = "%Y-%m-%dT00:00:00Z";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Date(NaiveDate);

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        date.0
    }
}

impl FromStr for Date {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(datetime) = DateTime::parse_from_rfc3339(s) {
            Ok(Date(datetime.date_naive()))
        } else {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(Date)
        }
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // SAFETY: midnight (00:00:00) is always valid for any NaiveDate
        let dt = self
            .0
            .and_hms_opt(0, 0, 0)
            .expect("midnight is always valid");
        let dt_utc = Utc.from_utc_datetime(&dt);
        serializer.serialize_str(&dt_utc.format(SER_FORMAT).to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use serde_json;

    #[test]
    fn test_date_serialization() {
        let date = Date(NaiveDate::from_ymd_opt(2025, 12, 24).unwrap());
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, "\"2025-12-24T00:00:00Z\"");
    }

    #[test]
    fn test_date_deserialization_from_date() {
        let json = "\"2025-12-24\"";
        let date: Date = serde_json::from_str(json).unwrap();
        assert_eq!(date, Date(NaiveDate::from_ymd_opt(2025, 12, 24).unwrap()));
    }

    #[test]
    fn test_date_deserialization_from_datetime() {
        let json = "\"2025-12-24T10:30:00Z\"";
        let date: Date = serde_json::from_str(json).unwrap();
        assert_eq!(date, Date(NaiveDate::from_ymd_opt(2025, 12, 24).unwrap()));
    }

    #[test]
    fn test_date_deserialization_error() {
        let json = "\"2025/12/24\"";
        let result: Result<Date, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
