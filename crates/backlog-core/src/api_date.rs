use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Date wrapper for Backlog API date parameters
///
/// Automatically formats DateTime<Utc> as "yyyy-MM-dd" when converted to string,
/// which is the standard format expected by Backlog API endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiDate(DateTime<Utc>);

impl ApiDate {
    /// Create a new ApiDate from DateTime<Utc>
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }

    /// Get the underlying DateTime<Utc>
    pub fn into_datetime(self) -> DateTime<Utc> {
        self.0
    }

    /// Get a reference to the underlying DateTime<Utc>
    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl From<DateTime<Utc>> for ApiDate {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::new(datetime)
    }
}

impl From<ApiDate> for DateTime<Utc> {
    fn from(api_date: ApiDate) -> Self {
        api_date.into_datetime()
    }
}

impl fmt::Display for ApiDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl Serialize for ApiDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ApiDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let datetime = DateTime::<Utc>::deserialize(deserializer)?;
        Ok(Self::new(datetime))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_api_date_display() {
        let datetime = Utc.with_ymd_and_hms(2024, 6, 24, 12, 30, 45).unwrap();
        let api_date = ApiDate::new(datetime);

        assert_eq!(api_date.to_string(), "2024-06-24");
    }

    #[test]
    fn test_api_date_from_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 9, 0, 0).unwrap();
        let api_date: ApiDate = datetime.into();

        assert_eq!(api_date.to_string(), "2024-01-15");
        assert_eq!(api_date.into_datetime(), datetime);
    }

    #[test]
    fn test_api_date_serialization() {
        let datetime = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let api_date = ApiDate::new(datetime);

        let serialized = serde_json::to_string(&api_date).unwrap();
        assert_eq!(serialized, "\"2024-12-31\"");
    }

    #[test]
    fn test_api_date_ordering() {
        let date1 = ApiDate::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let date2 = ApiDate::new(Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap());

        assert!(date1 < date2);
        assert_eq!(date1, date1);
    }

    #[test]
    fn test_api_date_datetime_reference() {
        let datetime = Utc.with_ymd_and_hms(2024, 6, 24, 12, 30, 45).unwrap();
        let api_date = ApiDate::new(datetime);
        assert_eq!(*api_date.datetime(), datetime);
    }
}
