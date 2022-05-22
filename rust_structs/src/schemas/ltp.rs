use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltp {
    pub status: &'static str,
    pub data: HashMap<&'static str, LtpData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LtpData {
    pub instrument_token: u32,
    pub last_price: f64,
}

#[test]
fn test_ltp() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"success","data":{"NSE:INFY":{"instrument_token":408065,"last_price":1459.00},"NSE:SBIN":{"instrument_token":500112,"last_price":465.50},"NSE:HDFC":{"instrument_token":500010,"last_price":2209.85}}}"#;
    let deserialized: Ltp = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    let mut data: HashMap<&str, LtpData> = HashMap::new();
    data.extend([
        (
            "NSE:INFY",
            LtpData {
                instrument_token: 408065,
                last_price: 1459.00,
            },
        ),
        (
            "NSE:SBIN",
            LtpData {
                instrument_token: 500112,
                last_price: 465.50,
            },
        ),
        (
            "NSE:HDFC",
            LtpData {
                instrument_token: 500010,
                last_price: 2209.85,
            },
        ),
    ]);
    assert_eq!(
        deserialized,
        Ltp {
            status: "success",
            data: data,
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_ltp_empty_data() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"success","data":{}}"#;
    let deserialized: Ltp = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Ltp {
            status: "success",
            ..Ltp::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}
