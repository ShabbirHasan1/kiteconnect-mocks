use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub status: super::common::Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, QuoteData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<super::common::Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteData {
    pub instrument_token: u32,
    pub timestamp: String,
    pub last_trade_time: Option<String>,
    pub last_price: f64,
    pub last_quantity: i64,
    pub buy_quantity: i64,
    pub sell_quantity: i64,
    pub volume: i64,
    pub average_price: f64,
    pub oi: f64,
    pub oi_day_high: f64,
    pub oi_day_low: f64,
    pub net_change: f64,
    pub lower_circuit_limit: f64,
    pub upper_circuit_limit: f64,
    pub ohlc: super::ohlc::OhlcInner,
    pub depth: Depth,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Depth {
    pub buy: Vec<Pqo>,
    pub sell: Vec<Pqo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pqo {
    pub price: f64,
    pub quantity: i64,
    pub orders: i64,
}

#[test]
fn test_quote_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../quote.json").unwrap();
    let deserialized: Quote = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    let mut data: HashMap<String, QuoteData> = HashMap::new();
    data.insert(
        "NSE:INFY".to_owned(),
        QuoteData {
            instrument_token: 408065,
            timestamp: "2021-06-08 15:45:56".to_owned(),
            last_trade_time: Some("2021-06-08 15:45:52".to_owned()),
            last_price: 1412.95,
            last_quantity: 5,
            buy_quantity: 0,
            sell_quantity: 5191,
            volume: 7360198,
            average_price: 1412.47,
            oi: 0.0,
            oi_day_high: 0.0,
            oi_day_low: 0.0,
            net_change: 0.0,
            lower_circuit_limit: 1250.7,
            upper_circuit_limit: 1528.6,
            ohlc: super::ohlc::OhlcInner {
                open: 1396.0,
                high: 1421.75,
                low: 1395.55,
                close: 1389.65,
            },
            depth: Depth {
                buy: [
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                ]
                .to_vec(),
                sell: [
                    Pqo {
                        price: 1412.95,
                        quantity: 5191,
                        orders: 13,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                    Pqo {
                        price: 0.0,
                        quantity: 0,
                        orders: 0,
                    },
                ]
                .to_vec(),
            },
        },
    );
    assert_eq!(
        deserialized,
        Quote {
            status: super::common::Status::Success,
            data: Some(data),
            ..Quote::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_quote_no_instruments() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"success","data":{}}"#;
    let deserialized: Quote = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Quote {
            status: super::common::Status::Success,
            data: Some(HashMap::new()),
            ..Quote::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_quote_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: Quote = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Quote {
            status: super::common::Status::Error,
            data: None,
            message: Some("Error message".to_owned()),
            error_type: Some(super::common::Exception::GeneralException),
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}
