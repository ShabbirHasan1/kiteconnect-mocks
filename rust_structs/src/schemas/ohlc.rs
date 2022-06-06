use super::common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ohlc {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, OhlcData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OhlcData {
    pub instrument_token: u64,
    pub last_price: f64,
    pub ohlc: OhlcInner,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OhlcInner {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    #[test]
    fn test_ohlc_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../ohlc.json").unwrap();
        let deserialized: Ohlc = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        let mut data: HashMap<String, OhlcData> = HashMap::new();
        data.insert(
            "NSE:INFY".to_owned(),
            OhlcData {
                instrument_token: 408065,
                last_price: 1075.0,
                ohlc: OhlcInner {
                    open: 1085.8,
                    high: 1085.9,
                    low: 1070.9,
                    close: 1075.8,
                },
            },
        );
        assert_eq!(
            deserialized,
            Ohlc {
                status: Status::Success,
                data: Some(data),
                ..Ohlc::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_ohlc_multiple_instruments() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data = r#"{"status":"success","data":{"NSE:INFY":{"instrument_token":408065,"last_price":1459,"ohlc":{"open":1453,"high":1466.75,"low":1446.7,"close":1455.15}},"NSE:SBIN":{"instrument_token":500112,"last_price":465.5,"ohlc":{"open":454.85,"high":464,"low":454.15,"close":462.4}},"NSE:HDFC":{"instrument_token":500010,"last_price":2209.85,"ohlc":{"open":2165,"high":2212,"low":2152.3,"close":2201.6}}}}"#.to_owned();
        let deserialized: Ohlc = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        let mut data: HashMap<String, OhlcData> = HashMap::new();
        data.extend([
            (
                "NSE:INFY".to_owned(),
                OhlcData {
                    instrument_token: 408065,
                    last_price: 1459.0,
                    ohlc: OhlcInner {
                        open: 1453.0,
                        high: 1466.75,
                        low: 1446.7,
                        close: 1455.15,
                    },
                },
            ),
            (
                "NSE:SBIN".to_owned(),
                OhlcData {
                    instrument_token: 500112,
                    last_price: 465.5,
                    ohlc: OhlcInner {
                        open: 454.85,
                        high: 464.0,
                        low: 454.15000000000003,
                        close: 462.40000000000003,
                    },
                },
            ),
            (
                "NSE:HDFC".to_owned(),
                OhlcData {
                    instrument_token: 500010,
                    last_price: 2209.85,
                    ohlc: OhlcInner {
                        open: 2165.0,
                        high: 2212.0,
                        low: 2152.3,
                        close: 2201.6,
                    },
                },
            ),
        ]);
        assert_eq!(
            deserialized,
            Ohlc {
                status: Status::Success,
                data: Some(data),
                ..Ohlc::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_ohlc_no_instruments() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data = r#"{"status":"success","data":{}}"#.to_owned();
        let deserialized: Ohlc = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Ohlc {
                status: Status::Success,
                data: Some(HashMap::new()),
                ..Ohlc::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_ohlc_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: Ohlc = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Ohlc {
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
