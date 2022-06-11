use super::common::*;
use crate::utils::*;
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalMinute {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HistoricalMinuteData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalMinuteData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candles: Option<Vec<Candle>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    #[serde(with = "naive_date_time_timezone_from_str")]
    pub timestamp: DateTime<FixedOffset>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

impl Default for Candle {
    fn default() -> Self {
        Self {
            timestamp: FixedOffset::east(5 * 30 * 60)
                .ymd(1947, 1, 1)
                .and_hms(9, 15, 0),
            open: Default::default(),
            high: Default::default(),
            low: Default::default(),
            close: Default::default(),
            volume: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    use std::borrow::BorrowMut;
    #[test]
    fn test_historical_minute_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../historical_minute.json").unwrap();
        let deserialized: HistoricalMinute = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            HistoricalMinute {
                status: Status::Success,
                data: Some(HistoricalMinuteData {
                    candles: Some(vec![
                        Candle {
                            timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 15, 0),
                            open: 1704.5,
                            high: 1705.0,
                            low: 1699.25,
                            close: 1702.8000000000002,
                            volume: 2499,
                        },
                        Candle {
                            timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 16, 0),
                            open: 1702.0,
                            high: 1702.0,
                            low: 1698.15,
                            close: 1698.15,
                            volume: 1271,
                        },
                        Candle {
                            timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 17, 0),
                            open: 1698.15,
                            high: 1700.25,
                            low: 1698.0,
                            close: 1699.25,
                            volume: 831,
                        },
                        Candle {
                            timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 18, 0),
                            open: 1700.0,
                            high: 1700.0,
                            low: 1698.3000000000002,
                            close: 1699.0,
                            volume: 771,
                        },
                        Candle {
                            timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 19, 0),
                            open: 1699.0,
                            high: 1700.0,
                            low: 1698.1000000000001,
                            close: 1699.8000000000002,
                            volume: 543,
                        },
                        Candle {
                            timestamp: FixedOffset::east(19800).ymd(2017, 12, 15).and_hms(9, 20, 0),
                            open: 1699.8000000000002,
                            high: 1700.0,
                            low: 1696.55,
                            close: 1696.9,
                            volume: 802,
                        },
                    ],),
                },),
                ..HistoricalMinute::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_historical_minutes_oi_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: HistoricalMinute = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            HistoricalMinute {
                status: Status::Error,
                data: None,
                message: Some("Error message".to_owned()),
                error_type: Some(Exception::GeneralException),
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }
}
