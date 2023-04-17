//! Utilities for serde
use serde::{Deserialize, Deserializer, Serializer};
use url::Url;

pub mod serde_url {
    use serde::Deserialize;
    use url::Url;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(url.as_ref())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url_string = String::deserialize(deserializer)?;
        Url::parse(&url_string).map_err(serde::de::Error::custom)
    }
}

pub mod opt_serde_url {
    use super::*;

    pub fn serialize<S>(url: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match url {
            Some(url) => serializer.serialize_str(url.as_ref()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let url_string: Option<String> = Option::deserialize(deserializer)?;
        match url_string {
            Some(s) => Url::parse(&s).map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

pub mod serde_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ";

    pub fn serialize<S>(date: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(*date, 0).unwrap(),
            Utc,
        );
        let s = format!("{}", datetime.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let datetime = DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)?;
        Ok(datetime.timestamp())
    }
}

pub mod opt_serde_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ";

    pub fn serialize<S>(date: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let datetime = DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(*d, 0).unwrap(),
                    Utc,
                );
                let s = format!("{}", datetime.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let datetime = DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(datetime.timestamp()))
            }
            None => Ok(None),
        }
    }
}
