use super::common::*;
use crate::utils::*;
use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderTrades {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrderTradesData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderTradesData {
    pub trade_id: String,
    pub order_id: String,
    pub exchange: Exchanges,
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub product: Products,
    pub average_price: f64,
    pub quantity: i64,
    pub exchange_order_id: String,
    pub transaction_type: TransactionType,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub fill_timestamp: Option<NaiveDateTime>,
    #[serde(
        with = "optional_naive_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_timestamp: Option<NaiveTime>,
    #[serde(
        with = "optional_naive_date_time_from_str",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_timestamp: Option<NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use std::borrow::BorrowMut;
    #[test]
    fn test_order_trades_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../order_trades.json").unwrap();
        let deserialized: OrderTrades = simd_json::from_reader(jsonfile)?;
        println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderTrades {
                status: Status::Success,
                data: Some(vec![OrderTradesData {
                    trade_id: "10000000".to_owned(),
                    order_id: "200000000000000".to_owned(),
                    exchange: Exchanges::Mcx,
                    tradingsymbol: "GOLDPETAL21JUNFUT".to_owned(),
                    instrument_token: 58424839,
                    product: Products::Normal,
                    average_price: 4852.0,
                    quantity: 1,
                    exchange_order_id: "300000000000000".to_owned(),
                    transaction_type: TransactionType::Buy,
                    fill_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)),
                    order_timestamp: Some(NaiveTime::from_hms(16, 0, 36)),
                    exchange_timestamp: Some(NaiveDate::from_ymd(2021, 5, 31).and_hms(16, 0, 36)),
                },]),
                ..OrderTrades::default()
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_order_trades_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#
                .to_owned();
        let deserialized: OrderTrades = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderTrades {
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
