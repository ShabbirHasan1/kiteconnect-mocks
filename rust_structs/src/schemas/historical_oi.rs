use super::common::*;
use crate::utils::*;
use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalOi {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HistoricalOiData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalOiData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candles: Option<Vec<CandleWithOi>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandleWithOi {
    #[serde(
        with = "naive_date_time_timezone_from_str"
    )]
    pub timestamp: DateTime<FixedOffset>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub open_interest: u64,
}

impl Default for CandleWithOi {
    fn default() -> Self {
        CandleWithOi {
            timestamp: FixedOffset::east(5 * 30 * 60)
                    .ymd(1947, 1, 1)
                    .and_hms(9, 15, 0),
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0,
            open_interest: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};

    #[test]
    fn test_historical_oi_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../historical_oi.json").unwrap();
        let deserialized: HistoricalOi = serde_json::from_reader(jsonfile)?;
        println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            HistoricalOi {
                status: Status::Success,
                data: Some(
                    HistoricalOiData {
                        candles: Some(
                            vec![
                                   CandleWithOi {
                                       timestamp: FixedOffset::east(19800).ymd(2019, 12, 4).and_hms(9, 15, 0),
                                       open: 12009.9,
                                       high: 12019.35,
                                       low: 12001.25,
                                       close: 12001.5,
                                       volume: 163275,
                                       open_interest: 13667775,
                                   },
                                   CandleWithOi {
                                       timestamp: FixedOffset::east(19800).ymd(2019, 12, 4).and_hms(9, 16, 0),
                                       open: 12001.0,
                                       high: 12003.0,
                                       low: 11998.25,
                                       close: 12001.0,
                                       volume: 105750,
                                       open_interest: 13667775,
                                   },
                                   CandleWithOi {
                                       timestamp: FixedOffset::east(19800).ymd(2019, 12, 4).and_hms(9, 17, 0),
                                       open: 12001.0,
                                       high: 12001.0,
                                       low: 11995.1,
                                       close: 11998.55,
                                       volume: 48450,
                                       open_interest: 13758000,
                                   },
                                   CandleWithOi {
                                       timestamp: FixedOffset::east(19800).ymd(2019, 12, 4).and_hms(9, 18, 0),
                                       open: 11997.8,
                                       high: 12002.0,
                                       low: 11996.25,
                                       close: 12001.55,
                                       volume: 52875,
                                       open_interest: 13758000,
                                   },
                                   CandleWithOi {
                                       timestamp: FixedOffset::east(19800).ymd(2019, 12, 4).and_hms(9, 19, 0),
                                       open: 12002.35,
                                       high: 12007.0,
                                       low: 12001.45,
                                       close: 12007.0,
                                       volume: 52200,
                                       open_interest: 13758000,
                                   },
                                   CandleWithOi {
                                       timestamp: FixedOffset::east(19800).ymd(2019, 12, 4).and_hms(9, 20, 0),
                                       open: 12006.95,
                                       high: 12009.25,
                                       low: 11999.6,
                                       close: 11999.6,
                                       volume: 65325,
                                       open_interest: 13777050,
                                   },
                            ],
                        ),
                    },
                ),
                ..HistoricalOi::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_historical_oi_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: HistoricalOi = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            HistoricalOi {
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
