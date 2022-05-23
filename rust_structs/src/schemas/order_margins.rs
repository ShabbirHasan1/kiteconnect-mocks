use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderMargins {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrderMarginsData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderMarginsData {
    #[serde(rename = "type")]
    pub order_margins_type: OrderMarginTypes, //equity/commodity
    pub tradingsymbol: String, // Trading symbol of the instrument
    pub exchange: Exchanges,   // Name of the exchange
    pub span: f64,             // SPAN margins
    pub exposure: f64,         // Exposure margins
    pub option_premium: f64,   // Option premium
    pub additional: f64,       // Additional margins
    pub bo: f64,               // Cash credit
    pub cash: f64,             // Cash credit
    pub var: f64,              // VAR
    pub pnl: Pnl,              // realised and unrealised profit and loss
    pub total: f64,            // Total margin block
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pnl {
    pub realised: f64,
    pub unrealised: f64,
}

#[test]
fn test_order_margins_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../order_margins.json").unwrap();
    let deserialized: OrderMargins = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        OrderMargins {
            status: Status::Success,
            data: Some(vec![OrderMarginsData {
                order_margins_type: OrderMarginTypes::Equity,
                tradingsymbol: "INFY".to_owned(),
                exchange: Exchanges::NSE,
                span: 0.0,
                exposure: 0.0,
                option_premium: 0.0,
                additional: 0.0,
                bo: 0.0,
                cash: 0.0,
                var: 961.45,
                pnl: Pnl {
                    realised: 0.0,
                    unrealised: 0.0,
                },
                total: 961.45,
            },]),
            ..OrderMargins::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_order_margins_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: OrderMargins = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        OrderMargins {
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
