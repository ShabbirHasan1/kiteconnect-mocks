use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ltp {
    pub status: String,
    pub data: HashMap<String, LtpData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LtpData {
    pub instrument_token: i64,
    pub last_price: f64,
}

#[test]
fn test_ltp() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"success","data":{"NSE:INFY":{"instrument_token":408065,"last_price":1074.35},"NSE:SBIN":{"instrument_token":408065,"last_price":1074.35},"NSE:HDFC":{"instrument_token":408065,"last_price":1074.35}}}"#;
    let deserialized: Ltp = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    let mut data: HashMap<String, LtpData> = HashMap::new();
    data.extend([
        (
            "NSE:INFY".into(),
            LtpData {
                instrument_token: 408065,
                last_price: 1074.35,
            },
        ),
        (
            "NSE:SBIN".into(),
            LtpData {
                instrument_token: 408065,
                last_price: 1074.35,
            },
        ),
        (
            "NSE:HDFC".into(),
            LtpData {
                instrument_token: 408065,
                last_price: 1074.35,
            },
        ),
    ]);
    assert_eq!(
        deserialized,
        Ltp {
            status: "success".into(),
            data: data,
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}
