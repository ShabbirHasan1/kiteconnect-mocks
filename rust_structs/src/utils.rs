use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub mod optional_naive_time_from_str {
    use chrono::NaiveTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const T_FORMAT: &'static str = "%H:%M:%S";
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_naive_time_string: Option<String> = match Deserialize::deserialize(deserializer) {
            Ok(naive_time_string) => Some(naive_time_string),
            Err(_) => None,
        };

        match maybe_naive_time_string {
            Some(naive_time_string) => NaiveTime::parse_from_str(&*naive_time_string, T_FORMAT)
                .map(Some)
                .map_err(de::Error::custom),
            None => Ok(None),
        }
    }
    pub fn serialize<'de, S>(
        naive_time: &Option<NaiveTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *naive_time {
            Some(ref dt) => serializer
                .serialize_some(&dt.format(T_FORMAT).to_string())
                .map_err(ser::Error::custom),
            None => serializer.serialize_none(),
        }
    }
}

pub mod optional_naive_date_time_from_str {
    use chrono::NaiveDateTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_naive_date_time_string: Option<String> =
            match Deserialize::deserialize(deserializer) {
                Ok(naive_date_time_string) => Some(naive_date_time_string),
                Err(_) => None,
            };

        match maybe_naive_date_time_string {
            Some(naive_date_time_string) => {
                NaiveDateTime::parse_from_str(&naive_date_time_string, DT_FORMAT)
                    .map(Some)
                    .map_err(de::Error::custom)
            }
            None => Ok(None),
        }
    }
    pub fn serialize<'de, S>(
        naive_date_time: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *naive_date_time {
            Some(ref dt) => serializer
                .serialize_some(&dt.format(DT_FORMAT).to_string())
                .map_err(ser::Error::custom),
            None => serializer.serialize_none(),
        }
    }
}

pub mod naive_date_time_timezone_from_str {
    use chrono::{DateTime, FixedOffset};
    use serde::{de, ser, Deserialize, Deserializer};

    // "2017-12-15T09:15:00+0530"
    const DT_TZ_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%z";
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_time_tz_string = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&date_time_tz_string, DT_TZ_FORMAT).map_err(de::Error::custom)
    }
    pub fn serialize<'de, S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_str(&dt.to_rfc3339().replace("+05:30", "+0530"))
            .map_err(ser::Error::custom)
    }
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
    use serde::{Deserialize, Serialize};
    #[test]
    fn test_optional_naive_date_time_from_str() -> serde_json::Result<()> {
        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(
                with = "optional_naive_date_time_from_str",
                skip_serializing_if = "Option::is_none"
            )]
            pub order_timestamp: Option<NaiveDateTime>,
        }
        let raw_data = r#"{"order_timestamp":"2017-12-29 11:06:52"}"#;
        let deserialized: OrdersData = serde_json::from_str(&raw_data)?;
        // println!("{:?}", deserialized);
        assert_eq!(
            deserialized,
            OrdersData {
                order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 29).and_hms(11, 6, 52))
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        assert_eq!(raw_data, serialized);
        Ok(())
    }
    #[test]
    fn test_optional_naive_time_from_str() -> serde_json::Result<()> {
        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(
                with = "optional_naive_time_from_str",
                skip_serializing_if = "Option::is_none"
            )]
            pub order_timestamp: Option<NaiveTime>,
        }
        let raw_data = r#"{"order_timestamp":"11:06:52"}"#;
        let deserialized: OrdersData = serde_json::from_str(&raw_data)?;
        // println!("{:?}", deserialized);
        assert_eq!(
            deserialized,
            OrdersData {
                order_timestamp: Some(NaiveTime::from_hms(11, 6, 52))
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        assert_eq!(raw_data, serialized);
        Ok(())
    }
    #[test]
    fn test_naive_date_time_timezone_from_str() -> serde_json::Result<()> {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(with = "naive_date_time_timezone_from_str")]
            pub order_timestamp: DateTime<FixedOffset>,
        }
        let raw_data = r#"{"order_timestamp":"2017-12-15T09:15:00+0530"}"#;
        let deserialized: OrdersData = serde_json::from_str(&raw_data)?;
        // println!("{:?}", deserialized);
        assert_eq!(
            deserialized,
            OrdersData {
                order_timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 15, 0)
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        assert_eq!(raw_data, serialized);
        Ok(())
    }
}
