// use chrono::{DateTime, FixedOffset, NaiveDateTime};
// use std::error::Error;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, std::io::Error> {
    // Open the file in read-only mode with buffer.
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub mod optional_naive_date_from_str {
    use chrono::NaiveDate;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_naive_date_string: Option<String> = match Deserialize::deserialize(deserializer) {
            Ok(naive_date_string) => Some(naive_date_string),
            Err(_) => None,
        };

        match maybe_naive_date_string {
            Some(naive_date_string) => NaiveDate::parse_from_str(&naive_date_string, DT_FORMAT)
                .map(Some)
                .map_err(de::Error::custom),
            None => Ok(None),
        }
    }
    pub fn serialize<S>(naive_date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *naive_date {
            Some(ref dt) => serializer
                .serialize_some(&dt.format(DT_FORMAT).to_string())
                .map_err(ser::Error::custom),
            None => serializer.serialize_none(),
        }
    }
}

pub mod naive_date_from_str {
    use chrono::NaiveDate;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let naive_date_string: String = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&naive_date_string, DT_FORMAT).map_err(de::Error::custom)
    }
    pub fn serialize<S>(naive_date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_str(&naive_date.format(DT_FORMAT).to_string())
            .map_err(ser::Error::custom)
    }
}

pub mod optional_naive_time_from_str {
    use chrono::NaiveTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%H:%M:%S";
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_naive_time_string: Option<String> = match Deserialize::deserialize(deserializer) {
            Ok(naive_time_string) => Some(naive_time_string),
            Err(_) => None,
        };

        match maybe_naive_time_string {
            Some(naive_time_string) => NaiveTime::parse_from_str(&*naive_time_string, DT_FORMAT)
                .map(Some)
                .map_err(de::Error::custom),
            None => Ok(None),
        }
    }
    pub fn serialize<S>(naive_time: &Option<NaiveTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *naive_time {
            Some(ref dt) => serializer
                .serialize_some(&dt.format(DT_FORMAT).to_string())
                .map_err(ser::Error::custom),
            None => serializer.serialize_none(),
        }
    }
}

pub mod optional_naive_date_time_from_str {
    use chrono::NaiveDateTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
    pub fn serialize<S>(
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

pub mod naive_date_time_from_str {
    use chrono::NaiveDateTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let naive_date_time_string: String = Deserialize::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&naive_date_time_string, DT_FORMAT).map_err(de::Error::custom)
    }
    pub fn serialize<S>(naive_date_time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_str(&naive_date_time.format(DT_FORMAT).to_string())
            .map_err(ser::Error::custom)
    }
}

pub mod optional_naive_date_time_with_milli_or_micro_from_str {
    use chrono::NaiveDateTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_naive_date_time_with_milli_or_micro_string: Option<String> =
            match Deserialize::deserialize(deserializer) {
                Ok(naive_date_time_with_milli_or_micro_string) => {
                    Some(naive_date_time_with_milli_or_micro_string)
                }
                Err(_) => None,
            };

        match maybe_naive_date_time_with_milli_or_micro_string {
            Some(naive_date_time_with_milli_or_micro_string) => NaiveDateTime::parse_from_str(
                &naive_date_time_with_milli_or_micro_string,
                DT_FORMAT,
            )
            .map(Some)
            .map_err(de::Error::custom),
            None => Ok(None),
        }
    }
    pub fn serialize<S>(
        naive_date_time_with_milli_or_micro: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *naive_date_time_with_milli_or_micro {
            Some(ref dt) => serializer
                .serialize_some(&dt.format(DT_FORMAT).to_string())
                .map_err(ser::Error::custom),
            None => serializer.serialize_none(),
        }
    }
}

pub mod naive_date_time_with_milli_or_micro_from_str {
    use chrono::NaiveDateTime;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let naive_date_time_with_milli_or_micro_string: String =
            Deserialize::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&naive_date_time_with_milli_or_micro_string, DT_FORMAT)
            .map_err(de::Error::custom)
    }
    pub fn serialize<S>(
        naive_date_time_with_milli_or_micro: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_str(
                &naive_date_time_with_milli_or_micro
                    .format(DT_FORMAT)
                    .to_string(),
            )
            .map_err(ser::Error::custom)
    }
}

pub mod naive_date_time_timezone_from_str {
    use chrono::{DateTime, FixedOffset};
    use serde::{de, ser, Deserialize, Deserializer};

    // "2017-12-15T09:15:00+0530"
    const DT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_time_tz_string = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&date_time_tz_string, DT_FORMAT).map_err(de::Error::custom)
    }
    pub fn serialize<S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer
            .serialize_str(&dt.to_rfc3339().replace("+05:30", "+0530"))
            .map_err(ser::Error::custom)
    }
}

pub mod naive_date_time_timezone_from_timestamp {
    use chrono::{offset::LocalResult::Single, DateTime, FixedOffset, TimeZone};
    use serde::{de, ser, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let given_timestamp = Deserialize::deserialize(deserializer)?;
        if let Single(datetimeobject) = FixedOffset::east(19800).timestamp_opt(given_timestamp, 0) {
            Ok(datetimeobject)
        } else {
            Err(de::Error::custom("Invalid timestamp"))
        }
    }
    pub fn serialize<S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // serializer
        //     .serialize_str(&dt.timestamp().to_string())
        //     .map_err(ser::Error::custom)
        serializer
            .serialize_str(&dt.to_rfc3339())
            .map_err(ser::Error::custom)
    }
}

pub mod double_optional_naive_date_from_str {
    use chrono::NaiveDate;
    use serde::{de, ser, Deserialize, Deserializer};
    const DT_FORMAT: &str = "%Y-%m-%d";
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Option<NaiveDate>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_naive_date_time_string: Option<Option<String>> =
            match Deserialize::deserialize(deserializer) {
                Ok(Some(Some(naive_date_time_string))) => Some(Some(naive_date_time_string)),
                Ok(Some(None) | None) | Err(_) => None,
                // Ok(Some(None)) | Err(_) => None,
                // Ok(None) | Err(_) => None,
            };

        match maybe_naive_date_time_string {
            Some(Some(naive_date_time_string)) => {
                NaiveDate::parse_from_str(&naive_date_time_string, DT_FORMAT)
                    .map(Some)
                    .map(Some)
                    .map_err(de::Error::custom)
            }
            Some(None) | None => Ok(None),
        }
    }

    pub fn serialize<S>(
        naive_date_time: &Option<Option<NaiveDate>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match naive_date_time {
            None => serializer.serialize_unit(),
            Some(None) => serializer.serialize_none(),
            Some(Some(dt)) => serializer
                .serialize_some(&dt.format(DT_FORMAT).to_string())
                .map_err(ser::Error::custom),
        }
    }
}

#[must_use]
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    use serde::Deserialize;
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// This process is more efficient than creating a String in memory especially
// working with larger files.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub struct TryFromSliceError(());

// #[stable(feature = "try_from", since = "1.34.0")]
// impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]
// where
//     [T; N]: LengthAtMost32,
// {
//     type Error = TryFromSliceError;

//     fn try_from(slice: &[T]) -> Result<&[T; N], TryFromSliceError> {
//         if slice.len() == N {
//             let ptr = slice.as_ptr() as *const [T; N];
//             unsafe { Ok(&*ptr) }
//         } else {
//             Err(TryFromSliceError(()))
//         }
//     }
// }

// struct TryFromSliceError(());

pub fn slice_to_array_8<T>(slice: &[T]) -> Result<&[T; 8], TryFromSliceError> {
    if slice.len() == 8 {
        let ptr = slice.as_ptr() as *const [T; 8];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_12<T>(slice: &[T]) -> Result<&[T; 12], TryFromSliceError> {
    if slice.len() == 64 {
        let ptr = slice.as_ptr() as *const [T; 12];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_28<T>(slice: &[T]) -> Result<&[T; 28], TryFromSliceError> {
    if slice.len() == 28 {
        let ptr = slice.as_ptr() as *const [T; 28];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_32<T>(slice: &[T]) -> Result<&[T; 32], TryFromSliceError> {
    if slice.len() == 32 {
        let ptr = slice.as_ptr() as *const [T; 32];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_44<T>(slice: &[T]) -> Result<&[T; 44], TryFromSliceError> {
    if slice.len() == 44 {
        let ptr = slice.as_ptr() as *const [T; 44];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_64<T>(slice: &[T]) -> Result<&[T; 64], TryFromSliceError> {
    if slice.len() == 64 {
        let ptr = slice.as_ptr() as *const [T; 64];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_164<T>(slice: &[T]) -> Result<&[T; 164], TryFromSliceError> {
    if slice.len() == 164 {
        let ptr = slice.as_ptr() as *const [T; 164];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_184<T>(slice: &[T]) -> Result<&[T; 184], TryFromSliceError> {
    if slice.len() == 184 {
        let ptr = slice.as_ptr() as *const [T; 184];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}
pub fn slice_to_array_492<T>(slice: &[T]) -> Result<&[T; 492], TryFromSliceError> {
    if slice.len() == 492 {
        let ptr = slice.as_ptr() as *const [T; 492];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryFromSliceError(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
    use serde::{Deserialize, Serialize};
    use std::borrow::BorrowMut;
    #[test]
    fn test_optional_naive_date_time_from_str() -> std::result::Result<(), simd_json::Error> {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(
                with = "optional_naive_date_time_from_str",
                skip_serializing_if = "Option::is_none"
            )]
            pub order_timestamp: Option<NaiveDateTime>,
        }
        let mut raw_data = r#"{"order_timestamp":"2017-12-29 11:06:52"}"#.to_owned();
        let deserialized: OrdersData = simd_json::from_str(raw_data.borrow_mut())?;
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
    fn test_optional_naive_time_from_str() -> std::result::Result<(), simd_json::Error> {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(
                with = "optional_naive_time_from_str",
                skip_serializing_if = "Option::is_none"
            )]
            pub order_timestamp: Option<NaiveTime>,
        }
        let mut raw_data = r#"{"order_timestamp":"11:06:52"}"#.to_owned();
        let deserialized: OrdersData = simd_json::from_str(raw_data.borrow_mut())?;
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
    fn test_naive_date_time_timezone_from_str() -> std::result::Result<(), simd_json::Error> {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(with = "naive_date_time_timezone_from_str")]
            pub order_timestamp: DateTime<FixedOffset>,
        }
        let mut raw_data = r#"{"order_timestamp":"2017-12-15T09:15:00+0530"}"#.to_owned();
        let deserialized: OrdersData = simd_json::from_str(raw_data.borrow_mut())?;
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
    #[test]
    fn test_naive_date_from_str() -> std::result::Result<(), simd_json::Error> {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct OrdersData {
            #[serde(with = "optional_naive_date_from_str")]
            pub order_timestamp: Option<NaiveDate>,
        }
        let mut raw_data = r#"{"order_timestamp":"2017-12-15"}"#.to_owned();
        let deserialized: OrdersData = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:?}", deserialized);
        assert_eq!(
            deserialized,
            OrdersData {
                order_timestamp: Some(NaiveDate::from_ymd(2017, 12, 15))
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        assert_eq!(raw_data, serialized);
        Ok(())
    }
}

// use chrono::{Utc, TimeZone, NaiveDateTime, NaiveDate, FixedOffset};

// fn main() {
//     assert_eq!(Utc.timestamp(1431648000, 0).to_string(), "2015-05-15 00:00:00 UTC");
//     println!("{}",Utc.timestamp(1654760856, 0).to_string());
//     println!("{}", NaiveDateTime::from_timestamp(1654760856,0));
//     let date_1 = FixedOffset::east(19800)
//                 .ymd(2022, 6, 9)
//                 .and_hms(13, 17, 36);
//     println!("{}", date_1.timestamp().to_string());
//     assert_eq!(1654760856, date_1.timestamp());
//     println!("{:#?}", FixedOffset::east(19800).timestamp(1654760856,0));
//     assert_eq!(FixedOffset::east(19800).timestamp(1654760856,0).to_string(), "2022-06-09 13:17:36 +05:30");
// }
