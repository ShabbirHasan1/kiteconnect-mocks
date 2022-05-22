use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ltp {
    pub status: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "NSE:INFY")]
    pub nse_infy: NseInfy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NseInfy {
    pub instrument_token: i64,
    pub last_price: f64,
}

#[test]
fn test_ltp() -> Result<()> {
    let raw_data = r#"{
                        "status": "success",
                        "data": {
                            "NSE:INFY": {
                                "instrument_token": 408065,
                                "last_price": 1074.35
                            }
                        }
                    }"#;
    let deserialized: Ltp = serde_json::from_str(&raw_data)?;
    assert_eq!(&deserialized, & Ltp { status: "succcess".to_string(), data: { }});
    let serialized = serde_json::to_string(&deserialized).unwrap();
    assert_eq!(raw_data, serialized);
    Ok(())
}
