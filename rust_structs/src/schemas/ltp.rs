use super::common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltp {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, LtpData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LtpData {
    pub instrument_token: u64,
    pub last_price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    #[test]
    fn test_ltp_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../ltp.json").unwrap();
        let deserialized: Ltp = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        let mut data: HashMap<String, LtpData> = HashMap::new();
        data.insert(
            "NSE:INFY".to_owned(),
            LtpData {
                instrument_token: 408065,
                last_price: 1074.35,
            },
        );
        assert_eq!(
            deserialized,
            Ltp {
                status: Status::Success,
                data: Some(data),
                ..Ltp::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_ltp_multiple_instruments() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data = r#"{"status":"success","data":{"NSE:INFY":{"instrument_token":408065,"last_price":1459.00},"NSE:SBIN":{"instrument_token":500112,"last_price":465.50},"NSE:HDFC":{"instrument_token":500010,"last_price":2209.85}}}"#.to_owned();
        let deserialized: Ltp = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        let mut data: HashMap<String, LtpData> = HashMap::new();
        data.extend([
            (
                "NSE:INFY".to_owned(),
                LtpData {
                    instrument_token: 408065,
                    last_price: 1459.00,
                },
            ),
            (
                "NSE:SBIN".to_owned(),
                LtpData {
                    instrument_token: 500112,
                    last_price: 465.50,
                },
            ),
            (
                "NSE:HDFC".to_owned(),
                LtpData {
                    instrument_token: 500010,
                    last_price: 2209.85,
                },
            ),
        ]);
        assert_eq!(
            deserialized,
            Ltp {
                status: Status::Success,
                data: Some(data),
                ..Ltp::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_ltp_no_instruments() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data = r#"{"status":"success","data":{}}"#.to_owned();
        let deserialized: Ltp = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Ltp {
                status: Status::Success,
                data: Some(HashMap::new()),
                ..Ltp::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_ltp_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: Ltp = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Ltp {
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
