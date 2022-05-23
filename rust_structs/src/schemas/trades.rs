use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trades {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradesData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesData {
    pub trade_id: String,
    pub order_id: String,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub product: Products,
    pub average_price: f64,
    pub quantity: i64,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_order_id: Option<String>,
    pub transaction_type: TransactionType,
    pub fill_timestamp: String,
    pub order_timestamp: String,
    pub exchange_timestamp: String,
}

#[test]
fn test_trades_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../trades.json").unwrap();
    let deserialized: Trades = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Trades {
            status: Status::Success,
            data: Some(vec![
                TradesData {
                    trade_id: "10000000".to_owned(),
                    order_id: "200000000000000".to_owned(),
                    exchange: Exchanges::NSE,
                    tradingsymbol: "SBIN".to_owned(),
                    instrument_token: 779521,
                    product: Products::CNC,
                    average_price: 420.65,
                    quantity: 1,
                    exchange_order_id: Some("300000000000000".to_owned()),
                    transaction_type: TransactionType::BUY,
                    fill_timestamp: "2021-05-31 09:16:39".to_owned(),
                    order_timestamp: "09:16:39".to_owned(),
                    exchange_timestamp: "2021-05-31 09:16:39".to_owned(),
                },
                TradesData {
                    trade_id: "40000000".to_owned(),
                    order_id: "500000000000000".to_owned(),
                    exchange: Exchanges::CDS,
                    tradingsymbol: "USDINR21JUNFUT".to_owned(),
                    instrument_token: 412675,
                    product: Products::MIS,
                    average_price: 72.755,
                    quantity: 1,
                    exchange_order_id: Some("600000000000000".to_owned()),
                    transaction_type: TransactionType::BUY,
                    fill_timestamp: "2021-05-31 11:18:27".to_owned(),
                    order_timestamp: "11:18:27".to_owned(),
                    exchange_timestamp: "2021-05-31 11:18:27".to_owned(),
                },
                TradesData {
                    trade_id: "70000000".to_owned(),
                    order_id: "800000000000000".to_owned(),
                    exchange: Exchanges::MCX,
                    tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                    instrument_token: 58424839,
                    product: Products::NRML,
                    average_price: 4852.0,
                    quantity: 1,
                    exchange_order_id: Some("312115100078593".to_owned()),
                    transaction_type: TransactionType::BUY,
                    fill_timestamp: "2021-05-31 16:00:36".to_owned(),
                    order_timestamp: "16:00:36".to_owned(),
                    exchange_timestamp: "2021-05-31 16:00:36".to_owned(),
                },
                TradesData {
                    trade_id: "90000000".to_owned(),
                    order_id: "1100000000000000".to_owned(),
                    exchange: Exchanges::MCX,
                    tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                    instrument_token: 58424839,
                    product: Products::NRML,
                    average_price: 4852.0,
                    quantity: 1,
                    exchange_order_id: Some("1200000000000000".to_owned()),
                    transaction_type: TransactionType::BUY,
                    fill_timestamp: "2021-05-31 16:08:41".to_owned(),
                    order_timestamp: "16:08:41".to_owned(),
                    exchange_timestamp: "2021-05-31 16:08:41".to_owned(),
                },
            ]),
            ..Trades::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_trades_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: Trades = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        Trades {
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
